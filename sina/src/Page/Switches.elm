module Page.Switches exposing (view)

import Browser exposing (Document)
import Html exposing (a, br, button, h2, img, p, span, table, td, text, th, tr)
import Html.Attributes exposing (height, href, src, width)
import Html.Events exposing (onClick)
import Iso8601
import Layout exposing (template)
import Model exposing (Model, Msg(..))
import Time exposing (Month(..), utc)


view : Model -> Document Msg
view { front, switches, switchPage } =
    let
        frontInfo =
            case front of
                Just front_data ->
                    [ text "All times are in UTC."
                    , h2 [] [ text "Current Front" ]
                    , p
                        []
                        [ text "Name: "
                        , text front_data.who
                        , br [] []
                        , text "Started At: "
                        , text <| Iso8601.fromTime front_data.started_at
                        , br [] []
                        , img [ src front_data.img_url, width 64, height 64 ] []
                        ]
                    ]

                Nothing ->
                    []

        heading =
            tr
                []
                [ th [] []
                , th [] [ text "ID" ]
                , th [] [ text "Who" ]
                , th [] [ text "Start" ]
                , th [] [ text "End" ]
                ]

        formatMonth month =
            case month of
                Jan ->
                    "Jan"

                Feb ->
                    "Feb"

                Mar ->
                    "Mar"

                Apr ->
                    "Apr"

                May ->
                    "May"

                Jun ->
                    "Jun"

                Jul ->
                    "Jul"

                Aug ->
                    "Aug"

                Sep ->
                    "Sep"

                Oct ->
                    "Oct"

                Nov ->
                    "Nov"

                Dec ->
                    "Dec"

        format time =
            span
                []
                [ text <| String.pad 2 '0' <| String.fromInt <| Time.toHour utc time
                , text ":"
                , text <| String.pad 2 '0' <| String.fromInt <| Time.toMinute utc time
                , text " "
                , text <| formatMonth <| Time.toMonth utc time
                , text " "
                , text <| String.fromInt <| Time.toDay utc time
                , text " "
                , text <| String.fromInt <| Time.toYear utc time
                ]

        rowify data =
            let
                ended_at =
                    case data.ended_at of
                        Nothing ->
                            span [] []

                        Just time ->
                            format time
            in
            tr
                []
                [ td [] [ img [ src data.img_url, width 16, height 16 ] [] ]
                , td [] [ a [ href <| "/switches/" ++ data.id ] [ text <| String.slice 0 10 data.id ] ]
                , td [] [ text data.who ]
                , td [] [ format data.started_at ]
                , td [] [ ended_at ]
                ]

        contents =
            List.map rowify switches

        prevButton =
            if switchPage > 1 then
                button [ onClick PrevSwitchesPage ] [ text "<-" ]

            else
                span [] []

        nextButton =
            button [ onClick NextSwitchesPage ] [ text "->" ]

        body =
            [ table [] <| [ heading ] ++ contents
            , p
                []
                [ prevButton
                , text <| String.fromInt switchPage
                , nextButton
                ]
            ]
    in
    Layout.template "Switches"
        (frontInfo
            ++ body
        )
