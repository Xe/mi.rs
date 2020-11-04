module Main exposing (main)

import Browser exposing (sandbox)
import Html exposing (Html, button, div, ul, li, span, input, text)
import Html.Attributes exposing (autofocus, class, value)
import Html.Events exposing (onClick, onInput)
import Mi
import Mi.Switch


-- We added a new AddTodo message type.


type Msg
    = UpdateText String
    | AddTodo
    | RemoveTodo Int



-- We added a new property called todos, which is a list of strings.


type alias Model =
    { text : String
    , todos : List String
    }



-- We added (autofocus True), which is like the native HTML autofocus attribute.
-- We also added a button that triggers an onClick event when clicked which
-- passes an AddTodo message to the update function.


view : Model -> Html Msg
view model =
    div [ class "text-center" ]
        [ input [ onInput UpdateText, value model.text, autofocus True ] []
        , button [ onClick AddTodo, class "btn btn-primary" ] [ text "Add Todo" ]
        , ul [] (List.indexedMap (\index todo -> li [] [ text todo, span [onClick (RemoveTodo index)] [text " X"] ]) model.todos)
        ]


update : Msg -> Model -> Model
update msg model =
    case msg of
        UpdateText newText ->
            { model | text = newText }

        -- We append the model.text value to the end of our list of todo strings.
        AddTodo ->
            { model | text = "", todos = model.todos ++ [ model.text ] }

        RemoveTodo index ->
            let
                beforeTodos =
                    List.take index model.todos

                afterTodos =
                    List.drop (index + 1) model.todos

                newTodos =
                    beforeTodos ++ afterTodos
            in
            { model | todos = newTodos }



-- We set the todos property so that it's initially an empty list.


main : Program () Model Msg
main =
    sandbox
        { init = { text = "", todos = [] }
        , view = view
        , update = update
        }