module Model exposing (Model, Msg(..), init)

import Browser exposing (UrlRequest(..))
import Browser.Navigation as Nav
import Http
import Mi
import Route exposing (Route, routeParser)
import Url exposing (Url)
import Url.Parser as UrlParser


type alias Model =
    { navKey : Nav.Key
    , route : Maybe Route
    , token : Maybe String
    , tokenData : Maybe Mi.TokenData
    , error : Maybe String
    }


type Msg
    = ChangeUrl Url
    | ClickLink UrlRequest
    | UpdateToken String
    | SubmitToken
    | ValidateToken (Result Http.Error Mi.TokenData)
    | ClearError


init : () -> Url -> Nav.Key -> ( Model, Cmd msg )
init _ url key =
    ( { navKey = key
      , route = UrlParser.parse routeParser url
      , token = Nothing
      , tokenData = Nothing
      , error = Nothing
      }
    , Cmd.none
    )
