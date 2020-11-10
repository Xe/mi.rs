module Signup exposing (main)

import Browser
import Css exposing (..)
import Html exposing (Attribute, Html, button, div, h1, input, text)
import Html.Attributes exposing (id, style, type_)
import Html.Events exposing (onClick, onInput)


type alias User =
    { name : String
    , email : String
    , password : String
    , loggedIn : Bool
    }


initialModel : User
initialModel =
    { name = ""
    , email = ""
    , password = ""
    , loggedIn = False
    }


view : User -> Html Msg
view user =
    div [ style "margin" "auto" ]
        [ h1 [ style "padding-left" "3cm" ] [ text "Sign up" ]
        , Html.form formStyle
            [ div []
                [ text "Name"
                , input ([ id "name", type_ "text", onInput SaveName ] ++ inputStyle) []
                ]
            , div []
                [ text "Email"
                , input ([ id "email", type_ "email", onInput SaveEmail ] ++ inputStyle) []
                ]
            , div []
                [ text "Password"
                , input ([ id "password", type_ "password", onInput SavePassword ] ++ inputStyle) []
                ]
            , div []
                [ button
                    ([ type_ "submit", onClick Signup ] ++ buttonStyle)
                    [ text "Create my account" ]
                ]
            ]
        ]


formStyle : List (Attribute msg)
formStyle =
    [ style "border-radius" "5px"
    , style "background-color" "#f2f2f2"
    , style "padding" "20px"
    , style "width" "300px"
    ]


inputStyle : List (Attribute msg)
inputStyle =
    [ style "display" "block"
    , style "width" "260px"
    , style "padding" "12px 20px"
    , style "margin" "8px 0"
    , style "border" "none"
    , style "border-radius" "4px"
    ]


buttonStyle : List (Attribute msg)
buttonStyle =
    [ style "width" "300px"
    , style "background-color" "#397cd5"
    , style "color" "white"
    , style "padding" "14px 20px"
    , style "margin-top" "10px"
    , style "border" "none"
    , style "border-radius" "4px"
    , style "font-size" "16px"
    ]


type Msg
    = SaveName String
    | SaveEmail String
    | SavePassword String
    | Signup


update : Msg -> User -> User
update message user =
    case message of
        SaveName name ->
            { user | name = name }

        SaveEmail email ->
            { user | email = email }

        SavePassword password ->
            { user | password = password }

        Signup ->
            { user | loggedIn = True }


main : Program () User Msg
main =
    Browser.sandbox
        { init = initialModel
        , view = view
        , update = update
        }
