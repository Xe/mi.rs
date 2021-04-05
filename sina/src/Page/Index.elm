module Page.Index exposing (view)

import Browser exposing (Document)
import Html exposing (br, h2, img, p, s, span, text)
import Html.Attributes exposing (height, src, width)
import Iso8601
import Layout exposing (basic, template)
import Mi
import Mi.Switch exposing (Switch)


type alias Model a =
    { a
        | tokenData : Maybe Mi.TokenData
        , front : Maybe Switch
    }


view : Model a -> Document msg
view { tokenData, front } =
    case tokenData of
        Nothing ->
            basic "Login Required" []

        Just data ->
            template "Mi"
                ([ h2 [] [ text "Token Info" ]
                 , p
                    []
                    [ span
                        []
                        [ text "Subscriber: "
                        , text data.sub
                        , br [] []
                        , text "Token ID: "
                        , text data.jti
                        , br [] []
                        , text "Issuer: "
                        , text data.iss
                        , br [] []
                        , text "Audience: "
                        , text data.aud
                        ]
                    ]
                 ]
                    ++ (case front of
                            Just front_data ->
                                [ h2 [] [ text "Current Front" ]
                                , span
                                    []
                                    [ text "Name: "
                                    , text front_data.who
                                    , br [] []
                                    , text "Started At: "
                                    , text <| Iso8601.fromTime front_data.started_at
                                    , br [] []
                                    , img
                                        [ src front_data.img_url
                                        , width 64
                                        , height 64
                                        ]
                                        []
                                    ]
                                ]

                            Nothing ->
                                []
                       )
                )
