module Page.OrangeConnex exposing (viewList, viewPackage)

import Browser exposing (Document)
import Html exposing (a, br, button, h2, img, p, span, table, td, text, th, tr)
import Html.Attributes exposing (height, href, src, style, width)
import Html.Events exposing (onClick)
import Iso8601
import Layout exposing (template)
import Mi.PackageTracking.OrangeConnex exposing (Package, Trace)
import Model exposing (Msg(..))


type alias Model a =
    { a
        | ocTrackingID : Maybe String
        , ocPackages : Maybe (List Package)
        , ocTraces : Maybe (List Trace)
    }


viewList : Model a -> Document Msg
viewList { ocPackages } =
    case ocPackages of
        Nothing ->
            Layout.template "Loading" [ text "please wait..." ]

        Just packages ->
            let
                heading =
                    tr
                        []
                        [ th [] [ text "ID" ]
                        , th [] [ text "recieved" ]
                        ]

                stringFromBool value =
                    if value then
                        "True"

                    else
                        "False"

                rowify data =
                    tr
                        []
                        [ td
                            []
                            [ a
                                [ href <| "/packages/orangeconnex/" ++ data.tracking_number, onClick <| FetchOCTraces data.tracking_number ]
                                [ text data.tracking_number ]
                            ]
                        , td [] [ text <| stringFromBool data.recieved ]
                        ]

                contents =
                    List.map rowify packages
            in
            Layout.template "OrangeConnex Packages"
                [ table [] <| [ heading ] ++ contents ]


viewPackage : Model a -> Document Msg
viewPackage { ocTraces, ocTrackingID } =
    case ocTraces of
        Nothing ->
            Layout.template "Loading..." [ span [] [] ]

        Just traces ->
            let
                heading =
                    tr
                        []
                        [ th [] [ text "Message" ]
                        , th [] [ text "City" ]
                        , th [] [ text "Country" ]
                        ]

                rowify data =
                    tr
                        []
                        [ td [] [ text data.description ]
                        , td [] [ text <| Maybe.withDefault "" data.city ]
                        , td [] [ text data.country ]
                        ]

                contents =
                    List.map rowify traces

                title =
                    "Info on package " ++ Maybe.withDefault "" ocTrackingID
            in
            Layout.template title [ table [] <| [ heading ] ++ contents ]
