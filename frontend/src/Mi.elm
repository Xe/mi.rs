module Mi exposing (TokenData, errorToString, expectJson, request, tokenDecoder, tokenIntrospectURL)

import Http exposing (Error(..))
import Json.Decode as D
import Url.Builder as UB


type alias TokenData =
    { sub : String
    , jti : String
    , aud : String
    , iss : String
    }


tokenDecoder : D.Decoder TokenData
tokenDecoder =
    D.map4 TokenData
        (D.field "sub" D.string)
        (D.field "jti" D.string)
        (D.field "aud" D.string)
        (D.field "iss" D.string)


tokenIntrospectURL : String
tokenIntrospectURL =
    UB.absolute
        [ "api", "token", "info" ]
        []


request method token path body expect =
    Http.request
        { method = method
        , body = body
        , headers =
            [ Http.header "Authorization" token
            ]
        , url = path
        , expect = expect
        , timeout = Nothing
        , tracker = Nothing
        }


expectJson : (Result Http.Error a -> msg) -> D.Decoder a -> Http.Expect msg
expectJson toMsg decoder =
    Http.expectStringResponse toMsg <|
        \response ->
            case response of
                Http.BadUrl_ url ->
                    Err (Http.BadUrl url)

                Http.Timeout_ ->
                    Err Http.Timeout

                Http.NetworkError_ ->
                    Err Http.NetworkError

                Http.BadStatus_ metadata body ->
                    Err (Http.BadStatus metadata.statusCode)

                Http.GoodStatus_ metadata body ->
                    case D.decodeString decoder body of
                        Ok value ->
                            Ok value

                        Err err ->
                            Err (Http.BadBody (D.errorToString err))


errorToString : Http.Error -> String
errorToString err =
    case err of
        Timeout ->
            "Timeout exceeded"

        NetworkError ->
            "Network error"

        BadStatus st ->
            "Bad status code: " ++ String.fromInt st

        BadBody text ->
            "Unexpected response from api: " ++ text

        BadUrl url ->
            "Malformed url: " ++ url
