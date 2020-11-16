module Model exposing (Model, Msg(..), init)

import Browser exposing (UrlRequest(..))
import Browser.Navigation as Nav
import Http
import Mi
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
    }


type Msg
    = ChangeUrl Url
    | ClickLink UrlRequest
    | UpdateToken String
    | SubmitToken
    | FetchSwitch String
    | FetchSwitches
    | ValidateToken (Result Http.Error Mi.TokenData)
    | ValidateSwitchByID (Result Http.Error Switch)
    | ValidateFront (Result Http.Error Switch)
    | ValidateSwitches (Result Http.Error (List Switch))
    | ClearError


init : () -> Url -> Nav.Key -> ( Model, Cmd msg )
init _ url key =
    ( { navKey = key
      , route = UrlParser.parse routeParser url
      , token = Nothing
      , tokenData = Nothing
      , error = Nothing
      , front = Nothing
      , switchPage = 1
      , switches = []
      , webMentionPage = 1
      , webMentions = []
      , switchByID = Nothing
      , webMentionByID = Nothing
      }
    , Nav.pushUrl key "/login"
    )
