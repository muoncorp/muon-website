<template>
  <v-container>
    <h3>
      Notice
    </h3>
    <v-expansion-panels
      hover
      focusable
    >
      <v-expansion-panel
        v-for="(notice, i) in noticeList"
        :key="i"
      >
        <v-expansion-panel-header
          @click="fetchNotice(notice)"
        >
          <div>
            {{ notice.title }}
            <span
              v-if="notice.time"
              class="float-right"
            >
              {{ new Date(notice.time).toLocaleDateString() }}
            </span>
          </div>
        </v-expansion-panel-header>
        <v-expansion-panel-content>
          <div v-html="notice.content"></div>
        </v-expansion-panel-content>
      </v-expansion-panel>
    </v-expansion-panels>
  </v-container>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator'
  import { getFromServer, getFromServerWithNoCache } from '@/utils'
  import { marked } from 'marked'

  interface Notice {
    title: string,
    path: string,
    content?: string,
    time: number,
  }

  @Component
  export default class NoticeBoard extends Vue {
    noticeList: Notice[] = []

    mounted () {
      getFromServer('/api/notice').then(response => {
        this.noticeList = response.data
      }).catch(error => {
        console.error('failed to get notice!', error)
      })
    }

    fetchNotice (notice: Notice) {
      if (!notice.content && notice.path) {
        notice.content = 'Loading...'
        getFromServerWithNoCache(notice.path).then(response => {
          console.log(response.data)
          notice.content = marked.parse(response.data)
          this.noticeList = this.noticeList.slice()
        }).catch(error => {
          notice.content = 'Failed.'
          console.error('failed to get notice content!', error)
        })
      }
    }
  }
</script>
