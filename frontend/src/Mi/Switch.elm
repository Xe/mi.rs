module Mi.Switch exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Iso8601
import Json.Decode exposing (Decoder, field, int, map5, nullable, string)
import Time exposing (..)
import Url.Builder as UB

type alias Switch =
    { id : String
    , who : String
    , started_at : Posix
    , ended_at : Maybe Posix
    , duration : Maybe Int
    }


decoder : Decoder Switch
decoder =
    map5 Switch
        (field "id" string)
        (field "who" string)
        (field "started_at" Iso8601.decoder)
        (field "ended_at" (nullable Iso8601.decoder))
        (field "duration" (nullable int))


switchURL : String
switchURL =
    UB.absolute
        [ "api", "switches", "switch" ]
        []


idURL : String -> String
idURL id =
    UB.absolute
        [ "api","switches", "id", id ]
        []


frontURL : String
frontURL =
    UB.absolute
        ["api", "switches", "current" ]
        []


listURL : Int -> Int -> String
listURL limit page =
    UB.absolute
        ["api", "switches", "" ]
        [ UB.int "limit" limit
        , UB.int "page" page
        ]