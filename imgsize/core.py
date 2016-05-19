class UnknownSize(Exception):
    pass


class ImageSize(object):
    def __init__(self):
        self.formats = []

    def register(self, format):
        self.formats.append(format)

    def get_size(self, fobj):
        for format in self.formats:
            if format.match(fobj):
                return format.get_size(fobj)
        raise UnknownSize()
