module Main exposing (..)

import Browser
import Html exposing (Html, div, h1, img, p, pre, text)
import Html.Attributes exposing (src)
import Http
import Mi


type Model
    = Failure String
    | Loading
    | Success String


init : () -> ( Model, Cmd Msg )
init _ =
    ( Loading
    , Http.get
        { url = "/.within/botinfo"
        , expect = Http.expectString GotText
        }
    )


type Msg
    = GotText (Result Http.Error String)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotText result ->
            case result of
                Ok fullText ->
                    ( Success fullText, Cmd.none )

                Err why ->
                    ( Failure (Mi.errorToString why), Cmd.none )


view : Model -> Html Msg
view model =
    case model of
        Failure why ->
            div []
                [ h1 [] [ text "Error" ]
                , p [] [ text why ]
                ]

        Loading ->
            div [] [ h1 [] [ text "Loading" ] ]

        Success msg ->
            div []
                [ h1 [] [ text "Mi" ]
                , pre [] [ text msg ]
                ]


main : Program () Model Msg
main =
    Browser.element
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }
