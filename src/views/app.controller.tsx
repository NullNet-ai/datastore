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
        <Container title="Views">
          Hello World
          <img src="/Users/chaosumaru/Documents/Projects/Platforms/v7/platform/DB/API/datastore/upload/1b951babbc6852a3ed97fa76471001cb" />
        </Container>
      </main>
    );
  }
}
