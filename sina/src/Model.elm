module Model exposing (Model, Msg(..), get, init)

import Browser exposing (UrlRequest(..))
import Browser.Navigation as Nav
import Http
import Mi
import Mi.POSSE
import Mi.PackageTracking.OrangeConnex as OrangeConnex
import Mi.Switch exposing (Switch)
import Mi.WebMention exposing (WebMention)
import Route exposing (Route, routeParser)
import Url exposing (Url)
import Url.Parser as UrlParser


type alias Model =
    { navKey : Nav.Key
    , route : Maybe Route
    , token : Maybe String
    , tokenData : Maybe Mi.TokenData
    , error : Maybe String
    , front : Maybe Switch
    , switchPage : Int
    , switches : List Switch
    , webMentionPage : Int
    , webMentions : List WebMention
    , switchByID : Maybe Switch
    , webMentionByID : Maybe WebMention
    , post : Mi.POSSE.Post
    , ocTrackingID : Maybe String
    , ocPackages : Maybe (List OrangeConnex.Package)
    , ocTraces : Maybe (List OrangeConnex.Trace)
    }


get : Model -> String -> Http.Expect Msg -> Cmd Msg
get model url action =
    Mi.request
        "GET"
        (Maybe.withDefault "" model.token)
        url
        Http.emptyBody
        action


type Msg
    = ChangeUrl Url
    | ClickLink UrlRequest
    | UpdateToken String
    | SubmitToken
    | FetchSwitch String
    | NextSwitchesPage
    | PrevSwitchesPage
    | FetchOCPackages
    | FetchOCTraces String
    | ValidateToken (Result Http.Error Mi.TokenData)
    | ValidateSwitchByID (Result Http.Error Switch)
    | ValidateFront (Result Http.Error Switch)
    | ValidateSwitches (Result Http.Error (List Switch))
    | ValidateOCPackages (Result Http.Error (List OrangeConnex.Package))
    | ValidateOCTraces (Result Http.Error (List OrangeConnex.Trace))
    | ClearError


init : Maybe String -> Url -> Nav.Key -> ( Model, Cmd msg )
init token url key =
    ( { navKey = key
      , route = UrlParser.parse routeParser url
      , token = token
      , tokenData = Nothing
      , error = Nothing
      , front = Nothing
      , switchPage = 1
      , switches = []
      , webMentionPage = 1
      , webMentions = []
      , switchByID = Nothing
      , webMentionByID = Nothing
      , post = Mi.POSSE.init
      , ocTrackingID = Nothing
      , ocPackages = Nothing
      , ocTraces = Nothing
      }
    , Nav.pushUrl key "/login"
    )
