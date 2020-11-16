module Mi.WebMention exposing (WebMention, decoder, idURL, listURL)

import Json.Decode exposing (Decoder, field, map3, string)
import Url.Builder as UB


type alias WebMention =
    { id : String
    , source_url : String
    , target_url : String
    }


decoder : Decoder WebMention
decoder =
    map3 WebMention
        (field "id" string)
        (field "source_url" string)
        (field "target_url" string)


idURL : String -> String
idURL id =
    UB.absolute
        [ "api", "webmention", id ]
        []


listURL : Int -> Int -> String
listURL limit page =
    UB.absolute
        [ "api", "webmention" ]
        [ UB.int "limit" limit
        , UB.int "page" page
        ]
