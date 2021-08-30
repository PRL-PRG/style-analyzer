# Model report for file:///tmp/top-repos-quality-repos-sbvnzbri/linter.git HEAD ba3fbaf9d7b741e3432a05bca2d6178169f36cd4

### Dump

```json
{'created_at': '2021-08-30 04:24:02',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.7 kB',
 'tags': [],
 'uuid': '5786871a-cdf5-413c-bef6-2a10381e0a91',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sbvnzbri/linter.git ba3fbaf9d7b741e3432a05bca2d6178169f36cd4

# javascript
111 rules, avg.len. 6.2
## train
PPCR: 0.977468
### report
macro
{'f1-score': 0.7583961842497429,
 'precision': 0.7675649881267167,
 'recall': 0.7549684479437249,
 'support': 14142}
micro
{'f1-score': 0.9217225286380992,
 'precision': 0.9217225286380992,
 'recall': 0.9217225286380992,
 'support': 14142}
weighted
{'f1-score': 0.9165786071660343,
 'precision': 0.9146407941302314,
 'recall': 0.9217225286380992,
 'support': 14142}
### report_full
macro
{'f1-score': 0.7442417374280392,
 'precision': 0.7675649881267167,
 'recall': 0.7276451028917016,
 'support': 14468}
micro
{'f1-score': 0.9112198531981824,
 'precision': 0.9217225286380992,
 'recall': 0.9009538291401714,
 'support': 14468}
weighted
{'f1-score': 0.9062696277572299,
 'precision': 0.9153895982723065,
 'recall': 0.9009538291401714,
 'support': 14468}
## test
PPCR: 0.989848
### report
macro
{'f1-score': 0.6980677795894329,
 'precision': 0.6586305390030771,
 'recall': 0.7682392635337167,
 'support': 3900}
micro
{'f1-score': 0.8976923076923077,
 'precision': 0.8976923076923077,
 'recall': 0.8976923076923077,
 'support': 3900}
weighted
{'f1-score': 0.8926153958441935,
 'precision': 0.8942832509975905,
 'recall': 0.8976923076923077,
 'support': 3900}
### report_full
macro
{'f1-score': 0.6938853673122388,
 'precision': 0.6586305390030771,
 'recall': 0.7603458172828382,
 'support': 3940}
micro
{'f1-score': 0.8931122448979592,
 'precision': 0.8976923076923077,
 'recall': 0.8885786802030456,
 'support': 3940}
weighted
{'f1-score': 0.8881535862020298,
 'precision': 0.8947763814239426,
 'recall': 0.8885786802030456,
 'support': 3940}
```

## javascript
### Summary
67 rules, avg.len. 5.8

| | |
|-|-|
|Min support|136|
|Max support|2669|
|Min confidence|0.9200507402420044|
|Max confidence|0.9989774823188782|

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
| 1 | `  -1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2614.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 489.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1272.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 193.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 161.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.920. Support: 394.` |
| 7 | `  -1.diff_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 193.` |
| 8 | `  -1.diff_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1349.` |
| 9 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 10 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 2669.` |
| 11 | `  -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1311.` |
| 12 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 188.` |
| 13 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 20<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1061.` |
| 14 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 173.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {CALL} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 206.` |
| 16 | `  -1.reserved not in {(}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 396.` |
| 17 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.945. Support: 389.` |
| 18 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 579.` |
| 19 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.length ≤ 4<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 445.` |
| 20 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.996. Support: 136.` |
| 21 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 2566.` |
| 22 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 481.` |
| 23 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1271.` |
| 24 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 188.` |
| 25 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1269.` |
| 26 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 178.` |
| 27 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.927. Support: 365.` |
| 28 | `  -1.diff_col ≥ 6<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 179.` |
| 29 | `  -1.diff_col ≤ 5<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 1352.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1236.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 195.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1206.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 152.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {ARGUMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 163.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 187.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1423.` |
| 37 | `  -1.internal_type not in {Identifier}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1297.` |
| 38 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 188.` |
| 39 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1220.` |
| 40 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 150.` |
| 41 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {ARGUMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 174.` |
| 42 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 384.` |
| 43 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.961. Support: 350.` |
| 44 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 167.` |
| 45 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 1304.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 200.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 1234.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 190.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 610.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.length ≤ 4<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 415.` |
| 51 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 609.` |
| 52 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.length ≤ 4<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 390.` |
| 53 | `  -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1361.` |
| 54 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 187.` |
| 55 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 171.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {CALL} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 197.` |
| 57 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 376.` |
| 58 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1265.` |
| 59 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 220.` |
| 60 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 1225.` |
| 61 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 161.` |
| 62 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.label in {<newline>}<br>	∧ +1.roles in {ARGUMENT} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 189.` |
| 63 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.937. Support: 374.` |
| 64 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 631.` |
| 65 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.length ≤ 6<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 442.` |
| 66 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 2616.` |
| 67 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.length ≤ 6<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 446.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.791044776119403, "max_conf": 0.9989774823188782, "max_support": 2669, "min_conf": 0.9200507402420044, "min_support": 136, "num_rules": 67}}
```
</details>
