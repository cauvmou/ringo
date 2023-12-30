<script setup lang="ts">
import { ref } from "vue";
import {
   NForm,
   NFormItem,
   NInput,
   NButton,
   type FormInst,
   FormItemInst,
   FormItemRule,
   FormValidationError,
   useMessage,
   FormRules,
} from "naive-ui";

const formRef = ref<FormInst | null>(null);
const message = useMessage();
const formValue = ref({
   user: {
      name: "",
      password: "",
   },
});
const rules = ref({
   user: {
      name: {
         required: true,
         message: "Please input your nickname",
         trigger: "blur",
      },
      password: {
         required: true, // TODO: check this (or just rework this password thingy in general)
         message: "Please input the room password",
         trigger: ["input", "blur"],
      },
   },
});

function handleValidateClick(e: MouseEvent) {
   e.preventDefault();
   formRef.value?.validate((errors) => {
      if (!errors) {
         message.success("Valid");
      } else {
         console.log(errors);
         message.error("Invalid");
      }
   });
}
</script>

<template>
   <main>
      <div class="center">
         <img src="/public/image.png" alt="Ringo Logo">
         <n-form :model="formValue" ref="formRef" label-placement="top" class="form" :rules="rules">
            <n-form-item label="Nickname" path="nicknameValue">
               <n-input v-model:value="formValue.user.name" placeholder="Nickname" />
            </n-form-item>
            <n-form-item label="Password" path="passwordValue">
               <n-input v-model:value="formValue.user.password" placeholder="Password" type="password" show-password-on="click" />
            </n-form-item>
            <n-button
               :disabled="formValue.user.name == null || formValue.user.password == null"
               round
               type="primary"
               @click="handleValidateClick"
            >
               Join Room
            </n-button>
         </n-form>
      </div>
   </main>
</template>

<style scoped lang="scss">
main {
   display: flex;
   height: 100vh;
   justify-content: center;
   align-items: center;

   .center {
      display: flex;
      flex-direction: column;
      gap: 4em;

      img {
         height: 10em;
      }

      .form {
         display: flex;
         flex-direction: column;
         gap: 1em;
      }
   }
}
</style>
