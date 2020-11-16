module Main exposing (main)

import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation as Nav
import Html exposing (Html, a, br, button, div, h1, img, input, p, pre, span, text)
import Html.Attributes exposing (href, placeholder, src, value)
import Html.Events exposing (onClick, onInput)
import Http
import Layout
import Mi
import Mi.Switch
import Mi.WebMention
import Route exposing (Route(..), routeParser)
import Url exposing (Url)
import Url.Parser as UrlParser exposing ((</>))


{-| All of the data that the app can hold.
-}
type alias Model =
    { navKey : Nav.Key
    , route : Maybe Route
    , token : Maybe String
    , tokenData : Maybe Mi.TokenData
    , error : Maybe String
    }


init : () -> Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url key =
    ( { navKey = key
      , route = UrlParser.parse routeParser url
      , token = Nothing
      , tokenData = Nothing
      , error = Nothing
      }
    , Cmd.none
    )


type Msg
    = ChangeUrl Url
    | ClickLink UrlRequest
    | UpdateToken String
    | SubmitToken
    | ValidateToken (Result Http.Error Mi.TokenData)
    | ClearError


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateToken newToken ->
            ( { model | token = Just newToken }, Cmd.none )

        ChangeUrl url ->
            ( { model | route = UrlParser.parse routeParser url }, Cmd.none )

        SubmitToken ->
            ( model
            , Mi.request
                "GET"
                (Maybe.withDefault "" model.token)
                Mi.tokenIntrospectURL
                Http.emptyBody
                (Mi.expectJson ValidateToken Mi.tokenDecoder)
            )

        ValidateToken result ->
            case result of
                Ok data ->
                    ( { model | tokenData = Just data }
                    , Nav.pushUrl model.navKey "/"
                    )

                Err why ->
                    ( { model | error = Just <| Mi.errorToString why }, Cmd.none )

        ClickLink urlRequest ->
            case urlRequest of
                Internal url ->
                    ( model, Nav.pushUrl model.navKey <| Url.toString url )

                External url ->
                    ( model, Nav.load url )

        ClearError ->
            ( { model | error = Nothing, token = Nothing }, Cmd.none )


view : Model -> Document Msg
view model =
    case model.error of
        Nothing ->
            case Maybe.withDefault Index model.route of
                Index ->
                    case model.tokenData of
                        Nothing ->
                            Layout.basic "Login Required" []

                        Just data ->
                            Layout.template "Mi"
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

                Login ->
                    Layout.basic "Login"
                        [ p [] [ text "Enter the secret code. Unauthorized access is prohibited." ]
                        , input [ placeholder "API Token", value (Maybe.withDefault "" model.token), onInput UpdateToken ] []
                        , button [ onClick SubmitToken ] [ text "Login" ]
                        ]

                _ ->
                    Debug.todo "implement routing"

        Just why ->
            Layout.basic
                "Error"
                [ p [] [ text why ]
                , a [ onClick ClearError, href "/" ] [ text "Clear error" ]
                ]


main : Program () Model Msg
main =
    Browser.application
        { view = view
        , init = init
        , update = update
        , subscriptions = always Sub.none
        , onUrlRequest = ClickLink
        , onUrlChange = ChangeUrl
        }
