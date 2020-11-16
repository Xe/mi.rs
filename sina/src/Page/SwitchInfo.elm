module Page.SwitchInfo exposing (view)

import Browser exposing (Document)
import Html exposing (br, img, p, span, text)
import Html.Attributes exposing (src)
import Layout exposing (template)
import Mi.Switch exposing (Switch)
import Page exposing (format)
import Time exposing (Month(..))


type alias Model a =
    { a | switchByID : Maybe Switch }


view : Model a -> Document msg
view { switchByID } =
    case switchByID of
        Nothing ->
            template "Loading" [ text "Please wait..." ]

        Just switch ->
            let
                title =
                    "Switch Details"

                ended_at =
                    case switch.ended_at of
                        Nothing ->
                            span [] []

                        Just time ->
                            span []
                                [ text "Ended at: "
                                , format
                                    time
                                , br [] []
                                ]
            in
            template title
                [ p
                    []
                    [ img [ src switch.img_url ] []
                    , br [] []
                    , text "ID: "
                    , text switch.id
                    , br [] []
                    , text "Name: "
                    , text switch.who
                    , br [] []
                    , text "Started At: "
                    , format switch.started_at
                    , br [] []
                    , ended_at
                    ]
                ]
