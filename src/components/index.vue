<template>
  <div class="common-layout">
    <el-container>

      <el-container>
        <el-header>

          <div>
            <el-link type="primary" href="https://joplin.sijiekeji.cn/shares/CUcBAGjpQlWZmSYC0kC1HC" target="_blank">
              帮助文档
            </el-link>

            <div class="textarea-button-wrapper">

              <el-input
                  v-model="userInput"
                  placeholder="请输入 SQL 如：SELECT * FROM 文件路径  【 更多请参考帮助文档 】"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 5 }"
              >
              </el-input>
              <el-button
                  class="inner-button"
                  type="success"
                  @click="getData(userInput)"
                  round
              >
                <i-ep-position style="padding-right: 2px"></i-ep-position>
                查询
              </el-button>
            </div>
          </div>
        </el-header>
        <el-main>
          <div id="container" style="width: 800px; height: 800px;"></div>


        </el-main>
        <el-footer>

        </el-footer
        >
      </el-container>
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from 'vue';
import {ListTable} from "@visactor/vtable";

const container = ref<HTMLElement | null>(null);
const userInput = ref('');

interface ResponseData {
  code: number;
  msg: string;
  data: {
    table_head: any[]; // Replace 'any' with the actual type of your table head data
    table_data: any[]; // Replace 'any' with the actual type of your table data
  };
}

function getData(sql: String) {
  invoke("file_data", {sql: sql}).then((data) => {
    const responseData = data as ResponseData; // Type assertion here

    if (responseData.code === 200) {
      let columns = responseData.data.table_head;
      let records = responseData.data.table_data;
      container.value = document.getElementById('container');
      if (container.value) {
        const tableInstance = new ListTable({
          container: container.value,
          records,
          columns,
          autoWrapText: true,
          autoFillWidth: true
        });
      }
    } else {
      ElMessage({
        showClose: true,
        message: responseData.msg,
        type: 'error',
      })
    }
  })

}
</script>

<style scoped>
.common-layout {
  height: 95vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;

}


.el-main {
  display: flex;
  flex-direction: column;
  overflow: auto;
  border: 1px solid #ccc;
  height: 300px;
  margin: 20px;
}

.el-footer {
  flex-shrink: 0;
  margin-bottom: 15px;
}


/** 对话框 */


.user-input input {
  width: 100%;
  padding: 5px;
  border: 1px solid #ccc;
  border-radius: 5px;
}

/** 文本域 按钮 */
.textarea-button-wrapper {
  position: relative;
}

.button-mt {
  margin-top: 10px;
}

.inner-button {
  position: absolute;
  right: 10px;
  bottom: 10px;
}
</style>


  