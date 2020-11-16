module Mi.Switch exposing (Switch, decoder, frontURL, idURL, listURL, switchURL)

import Iso8601
import Json.Decode exposing (Decoder, field, int, map6, nullable, string)
import Time exposing (Posix)
import Url.Builder as UB


type alias Switch =
    { id : String
    , who : String
    , started_at : Posix
    , ended_at : Maybe Posix
    , duration : Maybe Int
    , img_url : String
    }


decoder : Decoder Switch
decoder =
    map6 Switch
        (field "id" string)
        (field "who" string)
        (field "started_at" Iso8601.decoder)
        (field "ended_at" (nullable Iso8601.decoder))
        (field "duration" (nullable int))
        (field "img_url" string)


switchURL : String
switchURL =
    UB.absolute
        [ "api", "switches", "switch" ]
        []


idURL : String -> String
idURL id =
    UB.absolute
        [ "api", "switches", "id", id ]
        []


frontURL : String
frontURL =
    UB.absolute
        [ "api", "switches", "current" ]
        []


listURL : Int -> Int -> String
listURL limit page =
    UB.absolute
        [ "api", "switches", "" ]
        [ UB.int "limit" limit
        , UB.int "page" page
        ]
