module Page.Index exposing (view)

import Browser exposing (Document)
import Html exposing (br, p, span, text)
import Layout exposing (basic, template)
import Model exposing (Model)


view : Model -> Document msg
view { tokenData } =
    case tokenData of
        Nothing ->
            basic "Login Required" []

        Just data ->
            template "Mi"
                [ p
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
                        ]
                    ]
                ]
