import * as elements from 'typed-html';
import { Controller, Get, Header } from '@nestjs/common';
import { Container } from './components/container.html';

@Controller('view')
export class AppController {
  @Header('content-type', 'text/html')
  @Get()
  app() {
    return (
      <main class="flex flex-col gap-2 mx-2">
        <Container title="Views">Hello World</Container>
      </main>
    );
  }
}
