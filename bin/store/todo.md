# TODO

- [ ] Update convert_params_to_sql method in common_controller

does that mean whenever drain is triggered it goes to the flushing and then marks the channel as flushing and messages go to the queue, even when channel has not been backpressured

we want such mechanism that channel flush only triggers when channel has been backpressured

and why is drain necessary for the message flow even when channel has not been backpressured, you should modify it such a way that drain only occurs when channel has been backpressured before,

and in the flushing method, you flow is like this

mark channel as flushing remove it from backpressured, then check if there are any new messages, if not remove from both backpressured and flushing

now if there are messages you are getting 500 messages you consume all of them send to the channel, and delete all those ids from the stream items

then check if there are  any more messages if yes you should remove channel from the backpressured but keep it the flushing and queue the method of flushing again in the semaphore,

but if there aren't any you should remove channel from both backpressured and flushing

now the problem is whenver capacity goes from 18 to 20 if 20 is the capacity it triggers drain which doesn't make sense