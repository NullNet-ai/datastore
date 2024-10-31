import { Controller, Get, Res } from '@nestjs/common';
/**
 * Handles the GET request for the home route.
 * Redirects the user to the '/view' route with a 302 status code.
 *
 * @param res - The response object.
 */
@Controller()
export class AppController {
  @Get()
  home(@Res() res) {
    res.status(302).redirect('/view');
  }
}
