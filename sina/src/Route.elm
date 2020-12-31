module Route exposing (Route(..), routeParser)

import Url.Parser exposing ((</>), Parser, map, oneOf, s, string)


type Route
    = Index
    | Login
    | NotFound
    | SwitchLog
    | SwitchID String
    | MakeSwitch
    | WebMentionLog
    | WebMentionID String
    | Packages
    | OCPackages
    | OCPackage String


routeParser : Parser (Route -> a) a
routeParser =
    oneOf
        [ map Index <| s ""
        , map Login <| s "login"
        , map SwitchLog <| s "switches"
        , map SwitchID <| s "switches" </> string
        , map MakeSwitch <| s "switches" </> s "log"
        , map WebMentionLog <| s "webmentions"
        , map WebMentionID <| s "webmentions" </> string
        , map Packages <| s "packages"
        , map OCPackages <| s "packages" </> s "orangeconnex"
        , map OCPackage <| s "packages" </> s "orangeconnex" </> string
        ]
