class SCP:
    def __init__(self):
        self.transaction_log = []
        self.state = self.init_state()

    def init_state(self):
        """Subclasses should implement this to return a new empty state."""

        raise NotImplementedError

    def pre_validation(self, transaction):
        """Subclasses should implement this to discard bad transactions with
        respect to the current `self.state`."""

        raise NotImplementedError

    def apply(self, transactions):
        """Subclasses should implement this to update `self.state` with
        `transactions`."""

        raise NotImplementedError
