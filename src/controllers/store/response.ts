export class CustomResponse {
  // @ts-ignore
  private statusCode: number;
  private responseBody: any;
  private isEnded: boolean;
  private resolvePromise: (() => void) | null = null;
  private responsePromise: Promise<void>;

  constructor() {
    this.statusCode = 200; // Default status
    this.responseBody = null;
    this.isEnded = false;

    // Create a promise that resolves when send() is completed
    this.responsePromise = new Promise((resolve) => {
      this.resolvePromise = resolve;
    });
  }

  status(code: number): this {
    this.statusCode = code;
    return this;
  }

  json(body: any): this {
    return this.send(body); // Consistent behavior
  }

  send(body: any): this {
    if (this.isEnded) {
      console.error(
        '[CustomResponse] Cannot call send(): Response already ended.',
      );
      return this;
    }

    this.responseBody = body;

    this.end(); // Automatically call end

    // Resolve the response promise
    if (this.resolvePromise) {
      this.resolvePromise();
    }

    return this;
  }

  end(): this {
    if (this.isEnded) {
      return this;
    }

    this.isEnded = true;
    return this;
  }

  getBody(): any {
    return this.responseBody;
  }

  // Expose a method to wait until send() completes
  waitForResponse(): Promise<void> {
    return this.responsePromise;
  }
}
