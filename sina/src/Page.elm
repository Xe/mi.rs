module Page exposing (format)

import Html exposing (Html, span, text)
import Time exposing (Month(..), Posix, utc)


formatMonth : Month -> String
formatMonth month =
    case month of
        Jan ->
            "Jan"

        Feb ->
            "Feb"

        Mar ->
            "Mar"

        Apr ->
            "Apr"

        May ->
            "May"

        Jun ->
            "Jun"

        Jul ->
            "Jul"

        Aug ->
            "Aug"

        Sep ->
            "Sep"

        Oct ->
            "Oct"

        Nov ->
            "Nov"

        Dec ->
            "Dec"


format : Posix -> Html msg
format time =
    span
        []
        [ text <| String.pad 2 '0' <| String.fromInt <| Time.toHour utc time
        , text ":"
        , text <| String.pad 2 '0' <| String.fromInt <| Time.toMinute utc time
        , text " "
        , text <| formatMonth <| Time.toMonth utc time
        , text " "
        , text <| String.pad 2 '0' <| String.fromInt <| Time.toDay utc time
        , text " "
        , text <| String.fromInt <| Time.toYear utc time
        ]
