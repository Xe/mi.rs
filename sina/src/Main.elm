module Main exposing (main)

import Browser
import Html exposing (Html, div, h1, img, p, pre, text)
import Html.Attributes exposing (src)
import Http
import Layout
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


view : Model -> Browser.Document msg
view model =
    case model of
        Failure why ->
            Layout.template "Error"
                [ p [] [ text why ]
                ]

        Loading ->
            Layout.template "Loading" []

        Success msg ->
            Layout.template "Mi"
                [ pre [] [ text msg ]
                ]


main : Program () Model Msg
main =
    Browser.document
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        }
