// @ts-nocheck
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
import { AuthGuard } from '@dna-platform/crdt-lww-postgres';
import { FileInterceptor } from '@nestjs/platform-express';
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
    private storeQuery: StoreQueryDriver,
    private storeMutation: StoreMutationDriver,
  ) {}
  @Get('/:table/:id')
  async get(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.get(_res, _req);
  }

  @Get('/:table')
  async count(@Res() _res: Response, @Req() _req: Request) {
    return this.storeQuery.getCount(_res, _req);
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
  async download(@Res() _res: Response, @Req() _req: Request) {
    _req.query = {
      pluck:
        'id,organization_id,fieldname,originalname,encoding,mimetype,destination,filename,path,size',
      ..._req.query,
    };
    return this.storeMutation.download(_res, _req);
  }
}

// @UseGuards(AuthGuard)
// @Controller('/api/transactions')
// export class TransactionController {
//   constructor(private storeMutation: StoreMutationDriver) {}
//   @Post('/')
//   async transactions(@Res() _res: Response, @Req() _req: Request) {
//     return this.storeMutation.transactions(_res, _req);
//   }
// }

// @Controller('/api/test')
// export class TestController {
//   private db;
//   constructor(private drizzle: DrizzlePostgresProvider) {
//     this.db = DrizzlePostgresProvider.getDatabase();
//   }
//   @Get()
//   async test(@Res() _res: Response, @Req() _req: Request) {
//     console.log('hitting the test route....');
//     processHypertableQueries();
//     const results = await this.db.select().from(samples);

//     // const results = await this.db.execute(
//     //   sql`
//     //   SELECT time_bucket('1 hour', time) AS bucket,
//     //          AVG(temperature) AS avg_temperature,
//     //          MAX(temperature) AS max_temperature,
//     //          MIN(temperature) AS min_temperature
//     //   FROM samples
//     //   GROUP BY bucket
//     //   ORDER BY bucket;
//     // `,
//     // );
//     console.log(results);
//     _res.send(results);
//   }
// }
