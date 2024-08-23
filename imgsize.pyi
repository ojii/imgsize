from typing import Any, Iterable, TypedDict

class SizeDict(TypedDict):
    width: int
    height: int
    mime_type: int
    is_animated: int

class Size:
    width: int
    height: int
    mime_type: int
    is_animated: int

    def __init__(
        self, width: int, height: int, mime_type: str, is_animated: bool
    ) -> None: ...
    def as_dict(self) -> SizeDict: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: Any) -> bool: ...
    def __iter__(self) -> Iterable[int]: ...
    def __hash__(self) -> int: ...

def get_size(data: bytes) -> Size | None: ...
