

declare namespace Chat {
    // chat APi response
    interface ChatApiResponse{
       message:string
    }
    // 列表消息对话
    interface Message {
        id: number;
        // 内容
        content: string;
        // 是否用户信息
        isUserMessage: boolean;
      }

}