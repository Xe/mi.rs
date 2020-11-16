module Page.Login exposing (view)

import Browser exposing (Document)
import Html exposing (button, input, p, text)
import Html.Attributes exposing (placeholder, value)
import Html.Events exposing (onClick, onInput)
import Layout exposing (basic, template)
import Model exposing (Msg(..))


type alias Model a =
    { a | token : Maybe String }


view : Model a -> Document Msg
view model =
    basic "Login"
        [ p [] [ text "Enter the secret code. Unauthorized access is prohibited." ]
        , input [ placeholder "API Token", value (Maybe.withDefault "" model.token), onInput UpdateToken ] []
        , button [ onClick SubmitToken ] [ text "Login" ]
        ]
