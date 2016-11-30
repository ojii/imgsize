class UnknownSize(Exception):
    pass


class WrongFormat(Exception):
    pass


class ImageSize(object):
    def __init__(self):
        self.formats = []

    def register(self, format):
        self.formats.append(format)

    def get_size(self, fobj):
        for format in self.formats:
            try:
                return format(fobj)
            except WrongFormat:
                pass
            except UnknownSize:
                fobj.seek(0)
        raise UnknownSize()
