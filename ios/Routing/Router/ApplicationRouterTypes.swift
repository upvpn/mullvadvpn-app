//
//  ApplicationRouterTypes.swift
//  MullvadVPN
//
//  Created by pronebird on 17/08/2023.
//  Copyright © 2023 Mullvad VPN AB. All rights reserved.
//

import Foundation

/**
 Struct describing a routing request for presentation or dismissal.
 */
struct PendingRoute<RouteType: AppRouteProtocol>: Equatable {
    var operation: RouteOperation<RouteType>
    var animated: Bool
}

/**
 Enum type describing an attempt to fulfill the route presentation request.
 **/
enum PendingPresentationResult {
    /**
     Successfully presented the route.
     */
    case success

    /**
     The request to present this route should be dropped.
     */
    case drop

    /**
     The request to present this route cannot be fulfilled because the modal context does not allow
     for that.

     For example, on iPad, primary context cannot be presented above settings, because it enables
     access to settings by making the settings cog accessible via custom presentation controller.
     In such case the router will attempt to fulfill other requests in hope that perhaps settings
     can be dismissed first before getting back to that request.
     */
    case blockedByModalContext
}

/**
 Enum type describing an attempt to fulfill the route dismissal request.
 */
enum PendingDismissalResult {
    /**
     Successfully dismissed the route.
     */
    case success

    /**
     The request to present this route should be dropped.
     */
    case drop

    /**
     The route cannot be dismissed immediately because it's blocked by another modal presented
     above.

     The router will attempt to fulfill other requests first in hope to unblock the route by
     dismissing the modal above before getting back to that request.
     */
    case blockedByModalAbove
}

/**
 Enum describing operation over the route.
 */
enum RouteOperation<RouteType: AppRouteProtocol>: Equatable {
    /**
     Present route.
     */
    case present(RouteType)

    /**
     Dismiss route.
     */
    case dismiss(DismissMatch<RouteType>)

    /**
     Returns a group of affected routes.
     */
    var routeGroup: RouteType.RouteGroupType {
        switch self {
        case let .present(route):
            return route.routeGroup
        case let .dismiss(dismissMatch):
            return dismissMatch.routeGroup
        }
    }
}

/**
 Enum type describing a single route or a group of routes requested to be dismissed.
 */
enum DismissMatch<RouteType: AppRouteProtocol>: Equatable {
    case group(RouteType.RouteGroupType)
    case singleRoute(RouteType)

    /**
     Returns a group of affected routes.
     */
    var routeGroup: RouteType.RouteGroupType {
        switch self {
        case let .group(group):
            return group
        case let .singleRoute(route):
            return route.routeGroup
        }
    }
}

/**
 Struct describing presented route.
 */
public struct PresentedRoute<RouteType: AppRouteProtocol>: Equatable {
    public var route: RouteType
    public var coordinator: Coordinator
}

/**
 Struct holding information used by delegate to perform dismissal of the route(s) in subject.
 */
public struct RouteDismissalContext<RouteType: AppRouteProtocol> {
    /**
     Specific routes that are being dismissed.
     */
    public var dismissedRoutes: [PresentedRoute<RouteType>]

    /**
     Whether the entire group is being dismissed.
     */
    public var isClosing: Bool

    /**
     Whether transition is animated.
     */
    public var isAnimated: Bool
}

/**
 Struct holding information used by delegate to perform sub-navigation of the route in subject.
 */
public struct RouteSubnavigationContext<RouteType: AppRouteProtocol> {
    public var presentedRoute: PresentedRoute<RouteType>
    public var route: RouteType
    public var isAnimated: Bool
}
