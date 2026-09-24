from collections import deque
import heapq


class Order:
    def __init__(self, price: int, quantity: int):
        self.price = price
        self.quantity = quantity


class PriceBucket:
    def __init__(self, price: int):
        self.price = price
        self.orders: deque[Order] = deque()

    def add(self, order: Order):
        self.orders.append(order)

    def remove(self) -> Order | None:
        if self.orders:
            return self.orders.popleft()

    def peek(self) -> Order | None:
        if self.orders:
            return self.orders[0]


class BucketHeap:
    def __init__(self, reverse: bool = False):
        self.reverse = reverse
        self.buckets: dict[int, PriceBucket] = {}
        self.price_heap: list[int] = []

    def add(self, order: Order):
        if order.price not in self.buckets:
            self.buckets[order.price] = PriceBucket()
            self.price_heap = heapq.heappush(self.price_heap, order.price)
        self.buckets[order.price].add(order)

    def remove(self) -> Order | None:
        if self.price_heap:
            next = heapq.heappop(self.price_heap)
            return self.buckets[next].remove()

    def peek(self) -> Order | None:
        if self.price_heap:
            return self.buckets[0]
