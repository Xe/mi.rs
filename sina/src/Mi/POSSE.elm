module Mi.POSSE exposing (Post, decoder, encoder, init)

import Json.Decode as D
import Json.Encode as E


init : Post
init =
    { title = ""
    , body = ""
    , url = ""
    , tags = []
    }


type alias Post =
    { title : String
    , body : String
    , url : String
    , tags : List String
    }


encoder : Post -> E.Value
encoder post =
    E.object
        [ ( "title", E.string post.title )
        , ( "body", E.string post.body )
        , ( "url", E.string post.url )
        , ( "tags", E.list E.string post.tags )
        ]


decoder : D.Decoder Post
decoder =
    D.map4 Post
        (D.field "title" D.string)
        (D.field "body" D.string)
        (D.field "url" D.string)
        (D.field "tags" (D.list D.string))
