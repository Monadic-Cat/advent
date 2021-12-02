module Util
import Data.Maybe
import Data.Vect

-- The difference between `chunks` and `windows` is that
-- they produce non-overlapping and overlapping segments, respectively.

export
chunks : (n : Nat) -> List a -> List $ Vect n a
chunks x lst = fst $ takeChunks x Nil lst where
  takeChunk : (n : Nat) -> List a -> Maybe (Vect n a, List a)
  takeChunk 0 lst = Just (Nil, lst)
  takeChunk (S x) (first :: rest) =
    case takeChunk x rest of
         Just (v, lst) => Just (first :: v, lst)
         Nothing => Nothing
  takeChunk (S x) Nil = Nothing
  ||| Take all chunks from the input list, and also return trailing input.
  takeChunks : (n : Nat) -> List (Vect n a)  -> List a -> (List $ Vect n a, List a)
  takeChunks x vecs lst = case takeChunk x lst of
    Just (v, lst) => takeChunks x (vecs ++ [reverse v]) lst
    Nothing => (vecs, lst)

export
windows : (n : Nat) -> List a -> List $ Vect n a
windows x lst = takeWindows x Nil lst where
  takeWindow : (n : Nat) -> List a -> Maybe $ Vect n a
  takeWindow 0 lst = Just Nil
  takeWindow (S x) (first :: rest) =
    case takeWindow x rest of
         Just v => Just $ first :: v
         Nothing => Nothing
  takeWindow (S x) Nil = Nothing
  ||| Take all windows from the input list.
  takeWindows : (n : Nat) -> List (Vect n a)  -> List a -> List $ Vect n a
  takeWindows x vecs (first :: rest) = case takeWindow x (first :: rest) of
    Just v => takeWindows x (vecs ++ [reverse v]) rest
    Nothing => vecs
  takeWindows x vecs Nil = vecs


export
partial unwrap : Maybe a -> a
unwrap (Just x) = x
