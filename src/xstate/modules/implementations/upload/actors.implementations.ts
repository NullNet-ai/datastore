import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upload/upload.schema';
import { CreateActorsImplementations } from '../create/actors.implementations';
import { VerifyActorsImplementations } from '../verify';
// import * as Minio from 'minio';

const {
  UPLOAD_ENDPOINT,
  UPLOAD_ACCESS_KEY,
  UPLOAD_SECRET_KEY,
  // NODE_ENV,
  UPLOAD_BUCKET_NAME = 'test',
  UPLOAD_REGION = 'us-east-1',
  UPLOAD_PORT = '9000',
} = process.env;
@Injectable()
export class UploadActorsImplementations {
  // private client: Minio.Client;
  // File to upload
  sourceFile = '/tmp/test-file.txt';

  // Destination object name
  destinationObject = 'my-test-file.txt';
  constructor(
    private readonly createActorsImplementations: CreateActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {}

  async onModuleInit() {
    if (
      !UPLOAD_ENDPOINT ||
      !UPLOAD_ACCESS_KEY ||
      !UPLOAD_SECRET_KEY ||
      !UPLOAD_BUCKET_NAME ||
      !UPLOAD_REGION
    ) {
      throw new Error('Upload credentials not found');
    }
    console.log('UPLOAD_ENDPOINT', UPLOAD_ENDPOINT, UPLOAD_PORT);
    // this.client = new Minio.Client({
    //   endPoint: UPLOAD_ENDPOINT,
    //   port: +UPLOAD_PORT,
    //   useSSL: NODE_ENV === 'local',
    //   accessKey: UPLOAD_ACCESS_KEY,
    //   secretKey: UPLOAD_SECRET_KEY,
    //   // pathStyle: true,
    //   // region: UPLOAD_REGION,
    // });

    // // Check if the bucket exists
    // // If it doesn't, create it
    // const exists = await this.client.bucketExists(UPLOAD_BUCKET_NAME);
    // if (exists) {
    //   console.log(
    //     `Bucket ${UPLOAD_BUCKET_NAME} already exists in ${UPLOAD_REGION}.`,
    //   );
    // } else {
    //   await this.client.makeBucket(UPLOAD_BUCKET_NAME, UPLOAD_REGION, {
    //     // ObjectLocking: true,
    //   });
    //   console.log(
    //     `Bucket ${UPLOAD_BUCKET_NAME} created successfully in ${UPLOAD_REGION}.`,
    //   );
    // }
  }
  /**
   * Implementation of actors for the upload machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    verify: this.verifyActorImplementations.actors.verify,
    create: this.createActorsImplementations.actors.create,
    upload: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;

      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `No controller args found`,
            count: 0,
            data: [],
          },
        });
      const [_res, _req, _file] = context?.controller_args;
      console.log('@_file', _file);
      // Do some upload logic here
      // Set the object metadata
      // const metaData = {};
      // // Upload the file with fPutObject
      // // If an object with the same name exists,
      // // it is updated with new data
      // await this.client.fPutObject(
      //   UPLOAD_BUCKET_NAME,
      //   _file.filename,
      //   _req.url,
      //   metaData,
      // );

      return Promise.resolve({
        payload: {
          success: true,
          message: `File uploaded successfully to ${_req.url}`,
          count: 1,
          data: [_file],
        },
      });
    }),
  };
}
