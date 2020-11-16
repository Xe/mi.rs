module Route exposing (..)

import Url.Parser exposing ((</>), (<?>), Parser, int, map, oneOf, s, string)
import Url.Parser.Query as Query


type Route
    = Index
    | Login
    | NotFound
    | System
    | SwitchLog (Maybe Int)
    | SwitchID String
    | MakeSwitch
    | WebMentionLog (Maybe Int)
    | WebMentionID String


routeParser : Parser (Route -> a) a
routeParser =
    oneOf
        [ map Index (s "")
        , map Login (s "login")
        , map System (s "system")
        , map SwitchLog (s "switches" <?> Query.int "page")
        , map SwitchID (s "switches" </> string)
        , map MakeSwitch (s "switches" </> s "log")
        , map WebMentionLog (s "webmentions" <?> Query.int "page")
        , map WebMentionID (s "webmentions" </> string)
        ]
