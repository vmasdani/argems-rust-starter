module Main exposing (..)

import Browser
import Json.Decode.Pipeline as Pipeline
import Json.Decode as Decode
import Json.Encode as Encode
import Http
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import String exposing (fromInt)

type RequestStatus
  = Loading
  | Error
  | Success
  | NotAsked

type alias Model =
  { todos : List Todo 
  , requestStatus : RequestStatus
  , newTodo : Todo
  , debug : Bool
  , baseUrl : String
  }

type alias Todo =
  { id : Maybe Int
  , name : String
  , completed : Int
  , createdAt : Maybe String
  , updatedAt : Maybe String
  }
 
initialTodo : Todo
initialTodo =
  { id = Nothing
  , name = ""
  , completed = 0
  , createdAt = Nothing
  , updatedAt = Nothing
  }

type Msg
  = GotTodos (Result Http.Error (List Todo))
  | PostedTodos (Result Http.Error String)
  | ChangeTodoName String
  | InsertTodo
  | InvertDebug Bool
  | TodoDone Todo
  | DeleteTodo Int
  | DeletedTodo (Result Http.Error ())
  
todoDecoder : Decode.Decoder Todo
todoDecoder =
  Decode.succeed Todo
    |> Pipeline.required "id" (Decode.maybe Decode.int)
    |> Pipeline.required "name" Decode.string
    |> Pipeline.required "completed" Decode.int
    |> Pipeline.required "created_at" (Decode.maybe Decode.string)
    |> Pipeline.required "updated_at" (Decode.maybe Decode.string)

todoEncoder : Todo -> Encode.Value
todoEncoder todo =
  Encode.object
    [ ( "id"
      , case todo.id of 
          Just id ->
            Encode.int id

          _ ->
            Encode.null 
      )
    , ( "name", Encode.string todo.name )
    , ( "completed", Encode.int todo.completed )
    , ( "createdAt"
      , case todo.createdAt of 
          Just createdAt ->
            Encode.string createdAt

          _ ->
            Encode.null 
      )
    , ( "updatedAt"
      , case todo.updatedAt of 
          Just updatedAt ->
            Encode.string updatedAt

          _ ->
            Encode.null 
      )
    ]

main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }

init : String -> ( Model, Cmd Msg )
init baseUrl =
  ( { todos = [], requestStatus = Loading, newTodo = initialTodo, debug = False, baseUrl = baseUrl }
  , getTodos baseUrl
  )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    GotTodos res ->
      case res of
        Ok todos ->
          ( { model | requestStatus = Success, todos = todos }, Cmd.none )

        _ ->
          ( { model | requestStatus = Error }, Cmd.none )

    PostedTodos res ->
       case res of
        Ok _ ->
          ( { model | requestStatus = Success }, getTodos model.baseUrl )

        _ ->
          ( { model | requestStatus = Error }, Cmd.none )

    ChangeTodoName todoName ->
      let
        todo = model.newTodo
        newTodo = { todo | name = todoName }
      in
      ( { model | newTodo = newTodo }, Cmd.none )

    InsertTodo ->
      let
        todoToPost = model.newTodo
      in
      ( { model | newTodo = initialTodo }, postTodo model.baseUrl todoToPost )

    InvertDebug _ ->
      ( { model | debug = not model.debug }, Cmd.none )

    TodoDone todo ->
      ( { model | requestStatus = Loading }, postTodo model.baseUrl { todo | completed = if todo.completed == 1 then 0 else 1 } )

    DeleteTodo id ->
      ( model, deleteTodo model.baseUrl id )

    DeletedTodo _ ->
      ( model, getTodos model.baseUrl )

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

view : Model -> Html Msg
view model =
  div 
    [ style "display" "flex"
    , style "flex-direction" "column"
    , style "justify-content" "center"
    , style "align-items" "center"
    ]
    [ div []
        [ h1 [] [ text "ARGEMS starter" ] ]
    , div []
        [ h3 [] [ text "Todo App" ] ]
    , Html.form 
        [ onSubmit InsertTodo ]
        [ div []
            [ input 
                [ value model.newTodo.name
                , onInput ChangeTodoName
                , placeholder "Todo name..."
                ] []
            , button [ type_ "submit" ] [ text "Insert" ]
            ]  
        ]
       
    , div [] [ text "Click on name to finish todo" ]
    , div []
        ( List.map
            (\todo ->
              div 
                [ style "display" "flex"
                , style "border" "2px solid black" 
                , style "padding" "1em"
                , style "margin" "10px"
                , style "cursor" "pointer"
                , style "color"
                    ( if todo.completed == 1 then
                        "green"
                      
                      else
                        "red"
                    )
                , style "text-decoration"
                    ( if todo.completed == 1 then
                        "line-through"
                      
                      else
                        ""
                    )
                ]
                [ div [ onClick (TodoDone todo) ] [ text <| todo.name ]
                , button 
                    [ style "margin-left" "15px"
                    , onClick <| DeleteTodo (Maybe.withDefault 0 todo.id) 
                    ] 
                    [ text "Delete" ]
                ]
            )
            model.todos
        )
    , case model.requestStatus of
        Loading ->
          div [] [ text "Loading..." ]
        
        Error ->
          div [] [ text "Failure loading todos" ]

        _ ->
          div [] []
    ]
    
getTodos baseUrl =
  Http.get
    { url = baseUrl ++ "/todos"
    , expect = Http.expectJson GotTodos (Decode.list todoDecoder)
    }

postTodo baseUrl todo =
  Http.post
    { url = baseUrl ++ "/todos"
    , expect = Http.expectString PostedTodos
    , body = Http.jsonBody (todoEncoder todo)
    }

deleteTodo baseUrl id =
  Http.request
    { method = "DELETE"
    , headers = []
    , url =baseUrl ++ "/todos/" ++ String.fromInt id
    , body = Http.emptyBody
    , expect = Http.expectWhatever DeletedTodo
    , timeout = Nothing
    , tracker = Nothing
    }
