module Page.Login exposing (view)

import Browser exposing (Document)
import Html exposing (a, button, input, p, text)
import Html.Attributes exposing (href, placeholder, type_, value)
import Html.Events exposing (onClick, onInput)
import Layout exposing (basic)
import Model exposing (Msg(..))


type alias Model a =
    { a | token : Maybe String }


view : Model a -> Document Msg
view model =
    case model.token of
        Nothing ->
            basic "Login"
                [ p []
                    [ text "Unauthorized access is prohibited. See "
                    , a [ href "/login/gitea" ] [ text "here" ]
                    , text "to get a secret code."
                    ]
                , input
                    [ placeholder "API Token"
                    , value (Maybe.withDefault "" model.token)
                    , onInput UpdateToken
                    , type_ "password"
                    ]
                    []
                , button [ onClick SubmitToken, type_ "submit" ] [ text "Login" ]
                ]

        Just _ ->
            basic "Login"
                [ p []
                    [ text "Press Login to confirm your identity."
                    ]
                , button [ onClick SubmitToken, type_ "submit" ] [ text "Login" ]
                ]
