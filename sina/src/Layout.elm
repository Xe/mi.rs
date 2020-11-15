module Layout exposing (template)

import Browser exposing (Document)
import Html exposing (Html, a, div, h1, main_, nav, text)
import Html.Attributes exposing (class, href)


template : String -> List (Html msg) -> Document msg
template title body =
    { title = title
    , body =
        [ main_
            []
            ([ nav
                [ class "nav" ]
                [ a [ href "/" ] [ text "Mi" ]
                , text " - "
                , a [ href "/posse" ] [ text "POSSE" ]
                , text " - "
                , a [ href "/switches" ] [ text "Switches" ]
                , text " - "
                , a [ href "/webmentions" ] [ text "WebMentions" ]
                ]
             , h1 [] [ text title ]
             ]
                ++ body
            )
        ]
    }
