module Page.SwitchInfo exposing (view)

import Browser exposing (Document)
import Browser.Navigation as Nav
import Html exposing (a, br, button, h2, img, p, span, table, td, text, th, tr)
import Html.Attributes exposing (href, src, style)
import Html.Events exposing (onClick)
import Layout exposing (template)
import Model exposing (Model, Msg(..))
import Page exposing (format)
import Time exposing (Month(..), utc)


view : Model -> Document Msg
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
