<template>
  <div>
    <ImageHeader
      :src="require('../assets/contact.jpg')"
      title="Contact"
    />
    <v-container>
      <h2 id="contact">
        Contact Us
      </h2>
      <v-form
        ref="form"
        v-model="valid"
      >
        <v-container>
          <v-row>
            <v-col
              cols="12"
              md="6"
            >
              <v-text-field
                v-model="name"
                :rules="nameRules"
                :counter="20"
                label="Name*"
                required
              />
            </v-col>
            <v-col
              cols="12"
              md="6"
            >
              <v-text-field
                v-model="email"
                :rules="emailRules"
                :counter="64"
                label="E-mail*"
                required
              />
            </v-col>
          </v-row>
          <v-row>
            <v-col
              cols="12"
              md="6"
            >
              <v-text-field
                v-model="phone"
                :rules="phoneRules"
                :counter="15"
                label="Mobile number"
                required
              />
            </v-col>
            <v-col
              cols="12"
              md="6"
            >
              <v-text-field
                v-model="subject"
                :counter="100"
                label="Subject"
                required
              />
            </v-col>
          </v-row>
          <v-row>
            <v-col
              cols="12"
              md="12"
            >
              <v-textarea
                v-model="message"
                :counter="5000"
                label="Message"
                auto-grow
                required
              />
            </v-col>
          </v-row>
          <v-row
            justify="center"
          >
            <v-btn
              class="ma-2"
              :disabled="!valid || submitDisabled"
              @click="submit"
            >
              Submit
            </v-btn>
          </v-row>
        </v-container>
        <v-overlay :value="loading">
          <v-progress-circular
            indeterminate
            size="64"
          />
        </v-overlay>
      </v-form>
      <h2 id="location">
        Location
      </h2>
      <KakaoMap
        class="mb-2"
      />
      <h3>찾아 오시는 길</h3>
      <dl>
        <dt>
          <v-icon aria-hidden="false">mdi-map-marker</v-icon> 주소
        </dt>
        <dd>
          서울시 영등포구 양평로21가길 19, 선유도우림라이온스밸리 B동 304호
        </dd>
      </dl>
      <v-row>
        <v-col
          cols="12"
          md="4"
        >
          <dl>
            <dt>
              <v-icon aria-hidden="false">mdi-subway</v-icon> 지하철
              <v-chip
                color="#A17E46"
              >
                9
              </v-chip>호선 선유도역
            </dt>
            <dd>
              7번 출구에서 도보로 3분 거리
            </dd>
          </dl>
        </v-col>
        <v-col
          cols="12"
          md="4"
        >
          <dl>
            <dt>
              <v-icon aria-hidden="false">mdi-bus</v-icon> 한신아파트.선유도역 (19211)
            </dt>
            <dd>
              <v-chip
                x-small
                color="blue"
              >
                간선
              </v-chip>
              603<br>
              <v-chip
                x-small
                color="green"
              >
                지선
              </v-chip>
              5620, 6620<br>
              <v-chip
                x-small
                color="green"
              >
                마을
              </v-chip>
              양천01<br>
              <v-chip
                x-small
                color="blue"
              >
                좌석
              </v-chip>
              700
            </dd>
          </dl>
        </v-col>
        <v-col
          cols="12"
          md="4"
        >
          <dl>
            <dt>
              <v-icon aria-hidden="false">mdi-bus</v-icon> 한신아파트.선유도역 (19210)
            </dt>
            <dd>
              <v-chip
                x-small
                color="blue"
              >
                간선
              </v-chip>
              603<br>
              <v-chip
                x-small
                color="green"
              >
                지선
              </v-chip>
              5620, 6514, 6620<br>
              <v-chip
                x-small
                color="green"
              >
                마을
              </v-chip>
              양천01<br>
              <v-chip
                x-small
                color="blue"
              >
                좌석
              </v-chip>
              700
            </dd>
          </dl>
        </v-col>
      </v-row>
    </v-container>
    <v-dialog
      v-model="contactSuccessDialog"
      width="500"
    >
      <v-card>
        <v-card-title
          class="headline grey lighten-2"
          primary-title
        >
          담당자에게 메일이 전달되었습니다
        </v-card-title>
        <div class="ma-2 text-center">
          고객님의 소중한 의견에 감사드립니다.<br>
          내용 확인 후에 답장 메일 보내드리겠습니다.
        </div>
        <v-card-actions>
          <v-spacer />
          <v-btn
            color="primary"
            @click="contactSuccessDialog = false"
          >
            Close
          </v-btn>
          <v-spacer />
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog
      v-model="contactFailedDialog"
      width="500"
    >
      <v-card>
        <v-card-title
          class="headline grey lighten-2"
          primary-title
        >
          오류가 발생했습니다.
        </v-card-title>
        <div class="ma-2 text-center">
          메일 주소 <a href="mailto: contact@muon.co">contact@muon.co</a>로 문의하여 주세요.
        </div>
        <v-card-actions>
          <v-spacer />
          <v-btn
            color="primary"
            @click="contactFailedDialog = false"
          >
            Close
          </v-btn>
          <v-spacer />
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'
  import { postToServer } from '../utils'
  import KakaoMap from '@/components/KakaoMap.vue'

  @Component({
    components: {
      ImageHeader,
      KakaoMap,
    },
  })
  export default class ContactView extends Vue {
    loading = false
    contactSuccessDialog = false
    contactFailedDialog = false
    submitDisabled = false
    valid = false
    name = ''
    email = ''
    phone = ''
    subject = ''
    message = ''
    nameRules = [
      (v: any) => !!v || 'Name is required',
      (v: any) => v.length <= 20 || 'Name must be less than 20 characters',
    ]

    emailRules = [
      (v: any) => !!v || 'E-mail is required',
      (v: any) => v.length <= 64 || 'Name must be less than 64 characters',
      (v: any) => /.+@.+\..+/.test(v) || 'E-mail must be valid',
    ]

    phoneRules = [
      (v: any) => v.length <= 15 || 'Phone number must be less than 15 characters',
    ]

    mounted () {
      if (this.$route.hash) {
        this.$vuetify.goTo(this.$route.hash)
      }
    }

    reset () {
      this.name = ''
      this.email = ''
      this.phone = ''
      this.subject = ''
      this.message = ''
    }

    submit () {
      if ((this.$refs.form as any).validate()) {
        this.loading = true
        this.submitDisabled = true
        const data = {
          email: this.email,
          subject: `[From Contact Us] ${this.subject}`,
          text: `Name: ${this.name}\nE-mail: ${this.email}\nPhone number: ${this.phone}\n-----------------\n${this.message}`,
        }
        postToServer('/api/contact/send-message', data).then(response => {
          console.log('response', response)
          this.loading = false
          this.contactSuccessDialog = true
          this.submitDisabled = false
        }).catch(error => {
          console.error(error)
          this.loading = false
          this.contactFailedDialog = true
          this.submitDisabled = false
        })
      }
    }
  }
</script>

<style lang="scss" scoped>
dt {
  margin-top: 1rem;
}
dd {
  padding-left: 1rem;
}
</style>
