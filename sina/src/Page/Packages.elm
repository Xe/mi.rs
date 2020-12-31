module Page.Packages exposing (view)

import Browser exposing (Document)
import Html exposing (a, text)
import Html.Attributes exposing (href)
import Html.Events exposing (onClick)
import Iso8601
import Layout exposing (basic, template)
import Mi
import Model exposing (Msg(..))


view : Document Msg
view =
    template
        "Package Deliveries"
        [ a [ href "/packages/orangeconnex", onClick FetchOCPackages ] [ text "OrangeConnex" ] ]
