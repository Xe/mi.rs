module Mi.PackageTracking.OrangeConnex exposing (..)

import Json.Decode as D
import Json.Encode as E
import Url.Builder as UB


type alias Package =
    { tracking_number : String
    , recieved : Bool
    }


decodePackage : D.Decoder Package
decodePackage =
    D.map2 Package
        (D.field "tracking_number" D.string)
        (D.field "recieved" D.bool)


type alias Trace =
    { id : String
    , tracking_number : String
    , description : String
    , city : Maybe String
    , country : String
    , time_recorded : String
    , time_zone : String
    , ts : Int
    }


decodeTrace : D.Decoder Trace
decodeTrace =
    D.map8 Trace
        (D.field "id" D.string)
        (D.field "tracking_number" D.string)
        (D.field "description" D.string)
        (D.field "city" (D.nullable D.string))
        (D.field "country" D.string)
        (D.field "time_recorded" D.string)
        (D.field "time_zone" D.string)
        (D.field "ts" D.int)


packageListURL : String
packageListURL =
    UB.absolute
        [ "api", "packages", "orangeconnex" ]
        []


packageStatusURL : String -> String
packageStatusURL trackingNumber =
    UB.absolute
        [ "api", "packages", "orangeconnex", "status" ]
        [ UB.string "tn" trackingNumber ]


markRecievedURL : String -> String
markRecievedURL trackingNumber =
    UB.absolute
        [ "api", "packages", "orangeconnex", "delivered" ]
        [ UB.string "tn" trackingNumber ]
