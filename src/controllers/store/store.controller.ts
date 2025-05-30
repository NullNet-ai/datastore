import {
  Controller,
  Delete,
  Get,
  HttpStatus,
  Inject,
  ParseFilePipeBuilder,
  Patch,
  Post,
  Req,
  Res,
  UploadedFile,
  UploadedFiles,
  UseGuards,
  UseInterceptors,
  Put,
  Body,
  ForbiddenException,
} from '@nestjs/common';
import { Request, Response, Express } from 'express';
import {
  StoreMutationDriver,
  StoreQueryDriver,
  CustomCreateService,
  RootStoreService,
} from '../../providers/store/store.service';
import { AuthGuard } from '@dna-platform/crdt-lww-postgres';
import { FileInterceptor } from '@nestjs/platform-express';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import { AuthService as Auth } from '@dna-platform/crdt-lww-postgres/build/organizations/auth.service';
import { ValidateZod } from '../../decorator/validator';
import { z } from 'zod';
import {
  advanceFilterValidation,
  aggregationValidation,
  orderValidation,
} from '../../types/zod.types';
@UseGuards(AuthGuard)
@Controller('/api/store')
export class StoreController {
  constructor(
    @Inject('QueryDriverInterface')
    public storeQuery: StoreQueryDriver,
    public storeMutation: StoreMutationDriver,
  ) {}
  @Post('/:table/count')
  async count(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.getCount(_res, _req);
  }

  @Get('/:table/:id')
  async get(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.get(_res, _req);
  }

  @Post('/aggregate')
  @ValidateZod({
    body: z.object({
      entity: z.string().min(1, 'entity is required'),
      aggregations: z.array(aggregationValidation),
      advance_filters: z.array(advanceFilterValidation),
      bucket_size: z.string().min(1, 'bucket_size is required'),
      order: orderValidation,
    }),
  })
  async aggregate(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.aggregationFilter(_res, _req);
  }
  @Post('/batch/:table')
  async batchInsert(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.batchInsert(_res, _req);
  }

  @Patch('/batch/:table')
  async batchUpdate(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.batchUpdate(_res, _req);
  }

  @Post('/:table/filter')
  @ValidateZod({
    body: z.object({
      pluck: z.array(
        z
          .string()
          .min(1, "pluck fields is required, for getting all fields use ' * '"),
      ),
      advance_filters: z.array(advanceFilterValidation),
      order_by: z.string().optional(),
      limit: z.number().optional().default(50),
      offset: z.number().default(0),
      order_direction: z.enum(['asc', 'desc']).default('asc'),
    }),
  })
  async find(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.find(_res, _req);
  }

  @Post('/:table')
  async create(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.create(_res, _req);
  }

  @Patch('/:table/:id')
  async update(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.update(_res, _req);
  }

  @Delete('/:table/:id')
  async delete(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.delete(_res, _req);
  }

  @Post('/:table/filter/suggestions')
  @ValidateZod({
    body: z.object({
      pluck: z.array(
        z
          .string()
          .min(1, "pluck fields is required, for getting all fields use ' * '"),
      ),
      advance_filters: z.array(advanceFilterValidation),
      order_by: z.string().optional(),
      limit: z.number().optional().default(50),
      offset: z.number().default(0),
      order_direction: z.enum(['asc', 'desc']).default('asc'),
    }),
  })
  async searchSuggestions(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.searchSuggestions(_res, _req);
  }
}

@Controller('/api/hypertable')
export class HypertableController {
  constructor(private storeMutation: StoreMutationDriver) {}
  @Post()
  async createHypertables(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.createHypertables(_res, _req);
  }
}

@Controller('/api/token')
export class TokenController {
  constructor(private storeMutation: StoreMutationDriver) {}
  @Post('/verify')
  async create(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.verify(_res, _req);
  }
}

@UseGuards(AuthGuard)
@Controller('/api/file')
export class FileController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
  ) {}

  @Get('/:id')
  async get(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.getFileById(_res, _req);
  }

  @Post('/upload')
  @UseInterceptors(
    FileInterceptor('file', {
      // provide options later here
    }),
  )
  async upload(
    @Res() _res: Response,
    @Req() _req: Request,
    @UploadedFile()
    _file: Express.Multer.File,
  ) {
    return this.storeMutation.upload(_res, _req, _file);
  }

  @Post('/uploads')
  @UseInterceptors(
    FileInterceptor('files', {
      // provide options later here
    }),
  )
  async uploads(
    @Res() _res: Response,
    @Req() _req: Request,
    @UploadedFiles(
      new ParseFilePipeBuilder()
        .addFileTypeValidator({
          fileType: 'jpeg',
        })
        .addMaxSizeValidator({
          maxSize: 1000,
        })
        .build({
          errorHttpStatusCode: HttpStatus.UNPROCESSABLE_ENTITY,
        }),
    )
    _files: Array<Express.Multer.File>,
  ) {
    return this.storeMutation.uploads(_res, _req, _files);
  }

  @Get('/:id/download')
  async download(
    @Res()
    _res: Response,
    @Req() _req: Request,
  ) {
    const pluck = [
      'id',
      'organization_id',
      'fieldname',
      'originalname',
      'encoding',
      'mimetype',
      'destination',
      'filename',
      'path',
      'size',
      'presignedURL',
      'presignedURLExpires',
    ];
    _req.query = {
      pluck: pluck.join(','),
      ..._req.query,
    };
    return this.storeMutation.download(_res, _req);
  }
}
@Controller('/api/custom_create')
export class CustomCreateController {
  constructor(private customCreateService: CustomCreateService) {}
  @Post('/contact_emails')
  async createContactEmails(@Res() _res: Response, @Req() _req: Request) {
    return this.customCreateService.createContactEmail(_req.body);
  }
}

@Controller('/api/store/:type')
export class RootStoreController extends StoreController {
  constructor(
    @Inject('QueryDriverInterface')
    public storeQuery: StoreQueryDriver,
    public storeMutation: StoreMutationDriver,
    private readonly authService: AuthService,
    private readonly rootStoreService: RootStoreService,
    private readonly login: Auth,
  ) {
    super(storeQuery, storeMutation);
  }

  @Post('/switch_account')
  async switchAccount(
    @Body('data')
    data: {
      token: string;
      organization_id: string;
    },
    @Res() _res: Response,
  ) {
    const { account, signed_in_account = {} } = await this.authService.verify(
      data.token,
    );
    const { organization_id, account_id, account_organization_id } = account;

    const logged_account = await this.rootStoreService.getAccount({
      email: account_id,
      organization_id,
      account_organization_id,
    });

    if (!logged_account) {
      return _res.status(400).send({
        success: false,
        message: '[Switch Account]: Logged in account not found',
      });
    }

    const target_account = await this.rootStoreService.getAccount({
      email: account_id,
      organization_id: data.organization_id,
      account_id: logged_account.id,
    });

    const new_token_value = {
      account: target_account,
      signed_in_account,
      as_root: true,
    };
    const token = await this.login.sign(new_token_value);
    if (!token)
      throw new ForbiddenException('[Switch Account]: Token not generated');

    return _res.status(200).send({
      success: true,
      message: 'Account switched successfully',
      token,
    });
  }

  @Post('/switch_account_old')
  async switchAccountOld(
    @Body('data')
    data: {
      token: string;
      organization_id: string;
    },
    @Res() _res: Response,
  ) {
    const { account, signed_in_account = {} } = await this.authService.verify(
      data.token,
    );
    const {
      contact: account_contact,
      organization: account_organization,
      organization_account_id,
      account_id,
      is_external_user = false,
    } = account;

    const return_account_secret = true;
    const logged_account = await this.rootStoreService.getAccountOld({
      account_id,
      is_external_user,
      return_account_secret,
      organization_id: account_organization.id,
      contact_id: account_contact.id,
      organization_account_id,
    });

    if (!logged_account) {
      return _res.status(400).send({
        success: false,
        message: '[Switch Account]: Logged in account not found',
      });
    }

    const target_organization_account =
      await this.rootStoreService.getAccountOld({
        account_id: logged_account.account_id,
        return_account_secret,
        organization_id: data.organization_id,
      });

    const { account_secret, ...account_data } = target_organization_account;

    //! TODO: Check validy of logged in password and target account password
    // if (
    //   this.login.verify(logged_account.account_secret) &&
    //   this.login.verify(account_secret)
    // )
    //   throw new ForbiddenException('[Switch Account]: Account secret mismatch');

    const new_token_value = {
      account: account_data,
      signed_in_account,
      as_root: true,
    };
    const token = await this.login.sign(new_token_value);
    if (!token)
      throw new ForbiddenException('[Switch Account]: Token not generated');

    return _res.status(200).send({
      success: true,
      message: 'Account switched successfully',
      token,
    });
  }

  @Put('/:entity/password/:account_id')
  async password(@Res() res: Response, @Req() req: Request) {
    try {
      const { password } = req.body;
      const result = await this.rootStoreService.updatePassword(
        req.params.entity,
        {
          id: req.params.account_id,
          password,
        },
      );
      res.send(result);
    } catch (error: any) {
      res.send(error?.response || error);
    }
  }
}