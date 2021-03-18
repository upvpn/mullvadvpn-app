use super::{Error, Result, SettingsVersion};
use crate::wireguard::{MAX_ROTATION_INTERVAL, MIN_ROTATION_INTERVAL};
use std::time::Duration;


pub(super) struct Migration;

impl super::SettingsMigration for Migration {
    fn version_matches(&self, settings: &mut serde_json::Value) -> bool {
        settings
            .get("settings_version")
            .map(|version| version == SettingsVersion::V2 as u64)
            .unwrap_or(false)
    }

    fn migrate(&self, settings: &mut serde_json::Value) -> Result<()> {
        // `show_beta_releases` used to be nullable
        if settings
            .get_mut("show_beta_releases")
            .map(|val| val.is_null())
            .unwrap_or(false)
        {
            settings
                .as_object_mut()
                .ok_or(Error::NoMatchingVersion)?
                .remove("show_beta_releases");
        }

        let automatic_rotation = || -> Option<u64> {
            settings
                .get("tunnel_options")?
                .get("wireguard")?
                .get("automatic_rotation")
                .map(|ivl| ivl.as_u64())?
        }();

        if let Some(interval) = automatic_rotation {
            let new_ivl = match Duration::from_secs(60 * 60 * interval) {
                ivl if ivl < MIN_ROTATION_INTERVAL => {
                    log::warn!("Increasing key rotation interval since it is below minimum");
                    MIN_ROTATION_INTERVAL
                }
                ivl if ivl > MAX_ROTATION_INTERVAL => {
                    log::warn!("Decreasing key rotation interval since it is above maximum");
                    MAX_ROTATION_INTERVAL
                }
                ivl => ivl,
            };

            settings["tunnel_options"]["wireguard"]["rotation_interval"] =
                serde_json::json!(new_ivl);
        }

        settings["settings_version"] = serde_json::json!(SettingsVersion::V3);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::try_migrate_settings;
    use serde_json;

    const V2_SETTINGS: &str = r#"
{
  "account_token": "1234",
  "relay_settings": {
    "normal": {
      "location": {
        "only": {
          "country": "se"
        }
      },
      "tunnel": {
        "only": {
          "openvpn": {
            "port": {
              "only": 53
            },
            "protocol": {
              "only": "udp"
            }
          }
        }
      }
    }
  },
  "bridge_settings": {
    "normal": {
      "location": "any"
    }
  },
  "bridge_state": "auto",
  "allow_lan": true,
  "block_when_disconnected": false,
  "auto_connect": false,
  "tunnel_options": {
    "openvpn": {
      "mssfix": null
    },
    "wireguard": {
      "mtu": null,
      "automatic_rotation": 24
    },
    "generic": {
      "enable_ipv6": false
    }
  }
}
"#;

    pub const NEW_SETTINGS: &str = r#"
{
  "account_token": "1234",
  "relay_settings": {
    "normal": {
      "location": {
        "only": {
          "country": "se"
        }
      },
      "tunnel_protocol": "any",
      "wireguard_constraints": {
        "port": "any"
      },
      "openvpn_constraints": {
        "port": {
          "only": 53
        },
        "protocol": {
          "only": "udp"
        }
      }
    }
  },
  "bridge_settings": {
    "normal": {
      "location": "any"
    }
  },
  "bridge_state": "auto",
  "allow_lan": true,
  "block_when_disconnected": false,
  "auto_connect": false,
  "tunnel_options": {
    "openvpn": {
      "mssfix": null
    },
    "wireguard": {
      "mtu": null,
      "rotation_interval": {
          "secs": 86400,
          "nanos": 0
      }
    },
    "generic": {
      "enable_ipv6": false
    }
  },
  "settings_version": 3
}
"#;


    #[test]
    fn test_v2_migration() {
        let migrated_settings =
            try_migrate_settings(V2_SETTINGS.as_bytes()).expect("Migration failed");
        let new_settings = serde_json::from_str(NEW_SETTINGS).unwrap();

        assert_eq!(&migrated_settings, &new_settings);
    }
}
