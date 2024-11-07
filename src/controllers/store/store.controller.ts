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
} from '@nestjs/common';
import { Request, Response, Express } from 'express';
import {
  StoreMutationDriver,
  StoreQueryDriver,
} from '../../providers/store/store.service';
import { AuthGuard } from '@dna-platform/crdt-lww';
import { FileInterceptor } from '@nestjs/platform-express';
@UseGuards(AuthGuard)
@Controller('/api/store')
export class StoreController {
  constructor(
    @Inject('QueryDriverInterface')
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
  ) {}
  @Get('/:table/:id')
  async get(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.get(_res, _req);
  }

  @Post('/:table/filter')
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
}

@Controller('/api/token')
export class TokenController {
  constructor(private storeMutation: StoreMutationDriver) {}
  @Post('/verify')
  async create(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.verify(_res, _req);
  }
}

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

  @Post('/:id/download')
  async download(@Res() _res: Response, @Req() _req: Request) {
    return this.storeMutation.download(_res, _req);
  }
}
