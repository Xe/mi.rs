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
import Mi.PackageTracking.OrangeConnex
import Mi.Switch
import Mi.WebMention
import Model exposing (Model, Msg(..), get, init)
import Page.Index
import Page.Login
import Page.OrangeConnex
import Page.Packages
import Page.SwitchInfo
import Page.Switches
import Route exposing (Route(..), routeParser)
import Url
import Url.Parser as UrlParser


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        if_okay : Result Http.Error a -> (a -> ( Model, Cmd Msg )) -> ( Model, Cmd Msg )
        if_okay result doer =
            case result of
                Ok data ->
                    doer data

                Err why ->
                    ( { model | error = Just <| Mi.errorToString why }, Cmd.none )
    in
    case msg of
        UpdateToken newToken ->
            ( { model | token = Just newToken }, Cmd.none )

        ChangeUrl url ->
            ( { model | route = UrlParser.parse routeParser url }, Cmd.none )

        SubmitToken ->
            ( model
            , Cmd.batch
                [ get model Mi.tokenIntrospectURL <|
                    Mi.expectJson ValidateToken Mi.tokenDecoder
                , get model Mi.Switch.frontURL <|
                    Mi.expectJson ValidateFront Mi.Switch.decoder
                , get model (Mi.Switch.listURL 40 model.switchPage) <|
                    Mi.expectJson ValidateSwitches <|
                        Json.Decode.list Mi.Switch.decoder
                ]
            )

        FetchSwitch id ->
            ( model
            , get model (Mi.Switch.idURL id) <|
                Mi.expectJson ValidateSwitchByID Mi.Switch.decoder
            )

        NextSwitchesPage ->
            ( { model | switchPage = model.switchPage + 1 }
            , get model (Mi.Switch.listURL 40 <| model.switchPage + 1) <|
                Mi.expectJson ValidateSwitches <|
                    Json.Decode.list Mi.Switch.decoder
            )

        PrevSwitchesPage ->
            ( { model | switchPage = model.switchPage - 1 }
            , get model (Mi.Switch.listURL 40 <| model.switchPage - 1) <|
                Mi.expectJson ValidateSwitches <|
                    Json.Decode.list Mi.Switch.decoder
            )

        FetchOCPackages ->
            ( model
            , get model Mi.PackageTracking.OrangeConnex.packageListURL <|
                Mi.expectJson ValidateOCPackages <|
                    Json.Decode.list Mi.PackageTracking.OrangeConnex.decodePackage
            )

        FetchOCTraces trackingID ->
            ( { model | ocTrackingID = Just trackingID }
            , get model (Mi.PackageTracking.OrangeConnex.packageStatusURL trackingID) <|
                Mi.expectJson ValidateOCTraces <|
                    Json.Decode.list Mi.PackageTracking.OrangeConnex.decodeTrace
            )

        ValidateSwitchByID result ->
            if_okay result <|
                \data ->
                    ( { model | switchByID = Just data }, Cmd.none )

        ValidateSwitches result ->
            if_okay result <|
                \data ->
                    ( { model | switches = data }, Cmd.none )

        ValidateFront result ->
            if_okay result <|
                \data ->
                    ( { model | front = Just data }, Cmd.none )

        ValidateToken result ->
            if_okay result <|
                \data ->
                    ( { model | tokenData = Just data }
                    , Nav.pushUrl model.navKey "/"
                    )

        ValidateOCPackages result ->
            if_okay result <|
                \data ->
                    ( { model | ocPackages = Just data }, Cmd.none )

        ValidateOCTraces result ->
            if_okay result <|
                \data ->
                    ( { model | ocTraces = Just data }, Cmd.none )

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

                SwitchLog ->
                    Page.Switches.view model

                SwitchID _ ->
                    Page.SwitchInfo.view model

                Packages ->
                    Page.Packages.view

                OCPackages ->
                    Page.OrangeConnex.viewList model

                OCPackage packageID ->
                    Page.OrangeConnex.viewPackage { model | ocTrackingID = Just packageID }

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
