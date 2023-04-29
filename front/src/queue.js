class Queue {
    constructor(maxLength = 0) {
        this.elements = {};
        this.head = 0;
        this.tail = 0;
        this.maxLength = maxLength;
    }
    enqueue(element) {
        if (this.maxLength && this.length == this.maxLength) this.dequeue()
        this.elements[this.tail] = element;
        this.tail++;
    }
    dequeue() {
        const item = this.elements[this.head];
        delete this.elements[this.head];
        this.head++;
        return item;
    }
    peek() {
        return this.elements[this.head];
    }
    get length() {
        return this.tail - this.head;
    }
    get isEmpty() {
        return this.length === 0;
    }
}

export default Queue;