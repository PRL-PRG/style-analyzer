# Model report for file:///tmp/top-repos-quality-repos-p5_5lijk/leetcode-algorithm-practice.git HEAD 1a54196c7156f22f2e4b593126548f1a9f14b553

### Dump

```json
{'created_at': '2021-08-21 23:05:05',
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
 'size': '15.4 kB',
 'tags': [],
 'uuid': '16d9c6a3-4619-4d5b-a82f-4ba2fe0f42d9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-p5_5lijk/leetcode-algorithm-practice.git 1a54196c7156f22f2e4b593126548f1a9f14b553

# javascript
13 rules, avg.len. 4.7
## train
PPCR: 0.916135
### report
macro
{'f1-score': 0.7915352275621768,
 'precision': 0.7882853234127406,
 'recall': 0.7952320545572141,
 'support': 9766}
micro
{'f1-score': 0.9407126766332173,
 'precision': 0.9407126766332173,
 'recall': 0.9407126766332173,
 'support': 9766}
weighted
{'f1-score': 0.9370297728868884,
 'precision': 0.9336694390229245,
 'recall': 0.9407126766332173,
 'support': 9766}
### report_full
macro
{'f1-score': 0.7623163168389574,
 'precision': 0.7882853234127406,
 'recall': 0.7404928489199746,
 'support': 10660}
micro
{'f1-score': 0.8995398022128659,
 'precision': 0.9407126766332173,
 'recall': 0.8618198874296435,
 'support': 10660}
weighted
{'f1-score': 0.892578299458559,
 'precision': 0.9289845608699516,
 'recall': 0.8618198874296435,
 'support': 10660}
## test
PPCR: 0.921417
### report
macro
{'f1-score': 0.7880434041812142,
 'precision': 0.7835777236547465,
 'recall': 0.7934272767168524,
 'support': 2263}
micro
{'f1-score': 0.9350419796730004,
 'precision': 0.9350419796730004,
 'recall': 0.9350419796730004,
 'support': 2263}
weighted
{'f1-score': 0.929706998763967,
 'precision': 0.9248417288158598,
 'recall': 0.9350419796730004,
 'support': 2263}
### report_full
macro
{'f1-score': 0.7634931736077281,
 'precision': 0.7835777236547465,
 'recall': 0.7458797734817019,
 'support': 2456}
micro
{'f1-score': 0.8968001695274422,
 'precision': 0.9350419796730004,
 'recall': 0.8615635179153095,
 'support': 2456}
weighted
{'f1-score': 0.8892475196586568,
 'precision': 0.9206627323874654,
 'recall': 0.8615635179153095,
 'support': 2456}
```

## javascript
### Summary
7 rules, avg.len. 5.1

| | |
|-|-|
|Min support|99|
|Max support|4463|
|Min confidence|0.9555231928825378|
|Max confidence|0.9949495196342468|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 332.` |
| 2 | `  -3.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 471.` |
| 3 | `  -2.diff_col ≥ 7<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 102.` |
| 4 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.992. Support: 310.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 348.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 99.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, return, {, }}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 4463.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.142857142857143, "max_conf": 0.9949495196342468, "max_support": 4463, "min_conf": 0.9555231928825378, "min_support": 99, "num_rules": 7}}
```
</details>
