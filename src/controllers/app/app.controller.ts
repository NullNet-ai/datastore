import { Controller, Get, Res, Post, Body } from '@nestjs/common';
import { Response } from 'express';
import { AppService } from '../../providers/app/app.service';

/**
 * Handles the GET request for the home route.
 * Redirects the user to the '/view' route with a 302 status code.
 *
 * @param res - The response object.
 */
@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}
  @Get()
  home(@Res() res) {
    res.status(302).redirect('/view');
  }

  @Post('/api/sso/generate')
  generateSSOClient(
    @Body('application_name')
    application_name: string,
    @Res() res: Response,
  ) {
    const { client_id, client_secret } =
      this.appService.generateSSOClientDetails(application_name);
    res.status(200).send({
      success: true,
      message: 'Successfully generated SSO client details',
      data: [
        {
          client_id,
          client_secret,
        },
      ],
    });
  }
}
