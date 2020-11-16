module Main exposing (main)

import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation as Nav
import Html exposing (a, p, text)
import Html.Attributes exposing (href)
import Html.Events exposing (onClick)
import Http
import Json.Decode
import Layout
import Mi
import Mi.Switch
import Mi.WebMention
import Model exposing (Model, Msg(..), init)
import Page.Index
import Page.Login
import Route exposing (Route(..), routeParser)
import Url
import Url.Parser as UrlParser


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UpdateToken newToken ->
            ( { model | token = Just newToken }, Cmd.none )

        ChangeUrl url ->
            ( { model | route = UrlParser.parse routeParser url }, Cmd.none )

        SubmitToken ->
            ( model
            , Cmd.batch
                [ Mi.request
                    "GET"
                    (Maybe.withDefault "" model.token)
                    Mi.tokenIntrospectURL
                    Http.emptyBody
                    (Mi.expectJson ValidateToken Mi.tokenDecoder)
                , Mi.request
                    "GET"
                    (Maybe.withDefault "" model.token)
                    Mi.Switch.frontURL
                    Http.emptyBody
                    (Mi.expectJson ValidateFront Mi.Switch.decoder)
                , Mi.request
                    "GET"
                    (Maybe.withDefault
                        ""
                        model.token
                    )
                    (Mi.Switch.listURL 30 model.switchPage)
                    Http.emptyBody
                    (Mi.expectJson ValidateSwitches (Json.Decode.list Mi.Switch.decoder))
                ]
            )

        FetchSwitch id ->
            ( model
            , Mi.request
                "GET"
                (Maybe.withDefault "" model.token)
                (Mi.Switch.idURL id)
                Http.emptyBody
                (Mi.expectJson ValidateSwitchByID Mi.Switch.decoder)
            )

        FetchSwitches ->
            ( model
            , Mi.request
                "GET"
                (Maybe.withDefault
                    ""
                    model.token
                )
                (Mi.Switch.listURL 30 model.switchPage)
                Http.emptyBody
                (Mi.expectJson ValidateSwitches (Json.Decode.list Mi.Switch.decoder))
            )

        ValidateSwitchByID result ->
            case result of
                Ok data ->
                    ( { model | switchByID = Just data }, Cmd.none )

                Err why ->
                    ( { model | error = Just <| Mi.errorToString why }, Cmd.none )

        ValidateSwitches result ->
            case result of
                Ok data ->
                    ( { model | switches = data }, Cmd.none )

                Err why ->
                    ( { model | error = Just <| Mi.errorToString why }, Cmd.none )

        ValidateFront result ->
            case result of
                Ok data ->
                    ( { model | front = Just data }, Cmd.none )

                Err why ->
                    ( { model | error = Just <| Mi.errorToString why }, Cmd.none )

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
            ( { model | error = Nothing }, Cmd.none )


view : Model -> Document Msg
view model =
    case model.error of
        Nothing ->
            case Maybe.withDefault Index model.route of
                Index ->
                    Page.Index.view model

                Login ->
                    Page.Login.view model

                NotFound ->
                    Layout.template "Oh noes" [ p [] [ text "todo: implement this 404 page" ] ]

                System ->
                    Layout.template "System Info" [ p [] [ text "TODO(Ashe): implement this page" ] ]

                _ ->
                    Layout.template "Oh noes" [ p [] [ text "todo: implement this 404 page" ] ]

        Just why ->
            Layout.basic
                "Error"
                [ p [] [ text why, text ". Please clear the error to proceed." ]
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
