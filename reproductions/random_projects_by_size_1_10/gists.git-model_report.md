# Model report for file:///tmp/top-repos-quality-repos-0jf2kc9n/gists.git HEAD 8c22220605a8fa51965e56eebf815a92a637ec8a

### Dump

```json
{'created_at': '2021-08-22 07:39:30',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.2 kB',
 'tags': [],
 'uuid': 'a028f8a4-e85c-4482-8941-66bfb1ddbcc7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0jf2kc9n/gists.git 8c22220605a8fa51965e56eebf815a92a637ec8a

# javascript
51 rules, avg.len. 5.0
## train
PPCR: 0.862377
### report
macro
{'f1-score': 0.8549592399616052,
 'precision': 0.8840916052958367,
 'recall': 0.8451118949353545,
 'support': 4919}
micro
{'f1-score': 0.8877820695263264,
 'precision': 0.8877820695263264,
 'recall': 0.8877820695263264,
 'support': 4919}
weighted
{'f1-score': 0.8826399414390164,
 'precision': 0.8878462636750949,
 'recall': 0.8877820695263264,
 'support': 4919}
### report_full
macro
{'f1-score': 0.7752961489366282,
 'precision': 0.8840916052958367,
 'recall': 0.7201955436120125,
 'support': 5704}
micro
{'f1-score': 0.8221782923844487,
 'precision': 0.8877820695263264,
 'recall': 0.7656030855539971,
 'support': 5704}
weighted
{'f1-score': 0.808474610077091,
 'precision': 0.8880286542030339,
 'recall': 0.7656030855539971,
 'support': 5704}
## test
PPCR: 0.882687
### report
macro
{'f1-score': 0.8495992929615266,
 'precision': 0.8805978759081342,
 'recall': 0.8299721664884174,
 'support': 933}
micro
{'f1-score': 0.9163987138263665,
 'precision': 0.9163987138263665,
 'recall': 0.9163987138263665,
 'support': 933}
weighted
{'f1-score': 0.9127478872670837,
 'precision': 0.9133089388885935,
 'recall': 0.9163987138263665,
 'support': 933}
### report_full
macro
{'f1-score': 0.7797783300877613,
 'precision': 0.8805978759081342,
 'recall': 0.7277170070037092,
 'support': 1057}
micro
{'f1-score': 0.8592964824120601,
 'precision': 0.9163987138263665,
 'recall': 0.8088930936613056,
 'support': 1057}
weighted
{'f1-score': 0.846913199725438,
 'precision': 0.9093137313692294,
 'recall': 0.8088930936613056,
 'support': 1057}
```

## javascript
### Summary
28 rules, avg.len. 3.6

| | |
|-|-|
|Min support|153|
|Max support|911|
|Min confidence|0.927293062210083|
|Max confidence|0.9972222447395325|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 906.` |
| 2 | `  -1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.946. Support: 195.` |
| 3 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 282.` |
| 4 | `  -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 186.` |
| 5 | `  -1.reserved not in {(}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 173.` |
| 6 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 911.` |
| 7 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.943. Support: 183.` |
| 8 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.929. Support: 175.` |
| 9 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 298.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 447.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 12 | `  -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 153.` |
| 13 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 864.` |
| 14 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.975. Support: 179.` |
| 15 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.962. Support: 171.` |
| 16 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 172.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 18 | `  -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 172.` |
| 19 | `  +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.947. Support: 198.` |
| 20 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.947. Support: 179.` |
| 21 | `  -1.reserved not in {{}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 259.` |
| 22 | `  -1.reserved not in {(, ;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 23 | `  -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 185.` |
| 24 | `  -1.reserved not in {(}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 25 | `  -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 180.` |
| 26 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 27 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ +1.reserved not in {:, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 519.` |
| 28 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.943. Support: 167.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.642857142857143, "max_conf": 0.9972222447395325, "max_support": 911, "min_conf": 0.927293062210083, "min_support": 153, "num_rules": 28}}
```
</details>
